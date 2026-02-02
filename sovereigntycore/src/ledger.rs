use bincode::{serialize, deserialize};
use ed25519_dalek::{Keypair, PublicKey, Signer, Verifier, Signature};
use getrandom::getrandom;
use sha2::{Sha256, Digest};
use std::fs::{OpenOptions, File};
use std::io::{Write, Read, Seek, SeekFrom};
use std::path::Path;
use thiserror::Error;
use serde::{Serialize, Deserialize};

pub type Hash = [u8; 32];

#[derive(Error, Debug)]
pub enum LedgerError {
    #[error("Invalid RoH value: must be <= 0.3")]
    RoHExceeded,
    #[error("Monotone violation: new RoH > previous")]
    MonotoneViolation,
    #[error("Signature verification failed")]
    BadSignature,
    #[error("Chain hash mismatch")]
    ChainBroken,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Bincode error: {0}")]
    Bincode(#[from] bincode::Error),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToleranceEnvelope {
    pub pain_threshold: f64,      // higher = greater adult tolerance, no upper cap
    pub fear_threshold: f64,
    pub psych_risk_threshold: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DonutLoopEntry {
    pub prev_hash: Hash,
    pub timestamp: u64,
    pub roh: f64,
    pub tolerance: ToleranceEnvelope,
    pub event: String,
    pub pubkey: [u8; 32],
    pub signature: [u8; 64],
}

impl DonutLoopEntry {
    pub fn chain_hash(&self) -> Hash {
        let serialized = serialize(self).unwrap();
        Sha256::digest(serialized).into()
    }
}

pub struct DonutLoopLedger {
    path: String,
    brain_keypair: Keypair,
    current_roh: f64,
}

impl DonutLoopLedger {
    pub fn initialize(path: &str, initial_tolerance: ToleranceEnvelope) -> Result<Self, LedgerError> {
        let mut csprng = rand_core::OsRng {};
        let brain_keypair: Keypair = Keypair::generate(&mut csprng);

        let genesis = DonutLoopEntry::create(
            [0u8; 32],
            0.0, // initial safe state
            initial_tolerance,
            "genesis: sovereign identity established".to_string(),
            &brain_keypair,
        )?;

        let mut ledger = DonutLoopLedger {
            path: path.to_string(),
            brain_keypair,
            current_roh: 0.0,
        };
        ledger.append(&genesis)?;
        Ok(ledger)
    }

    pub fn append_event(
        &mut self,
        new_roh: f64,
        tolerance: ToleranceEnvelope,
        event: String,
    ) -> Result<(), LedgerError> {
        if new_roh > self.current_roh {
            return Err(LedgerError::MonotoneViolation);
        }
        if new_roh > 0.3 {
            return Err(LedgerError::RoHExceeded);
        }

        let prev_hash = self.last_entry_hash()?;
        let entry = DonutLoopEntry::create(prev_hash, new_roh, tolerance, event, &self.brain_keypair)?;
        self.append(&entry)?;
        self.current_roh = new_roh;
        Ok(())
    }

    fn append(&self, entry: &DonutLoopEntry) -> Result<(), LedgerError> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        let serialized = serialize(entry)?;
        let len = serialized.len() as u32;
        file.write_all(&len.to_be_bytes())?;
        file.write_all(&serialized)?;
        Ok(())
    }

    fn last_entry_hash(&self) -> Result<Hash, LedgerError> {
        if !Path::new(&self.path).exists() {
            return Ok([0u8; 32]);
        }
        let mut file = File::open(&self.path)?;
        let mut last_hash = [0u8; 32];
        let mut buffer = [0u8; 4];
        while let Ok(()) = file.read_exact(&mut buffer) {
            let len = u32::from_be_bytes(buffer) as usize;
            let mut entry_bytes = vec![0u8; len];
            file.read_exact(&mut entry_bytes)?;
            let entry: DonutLoopEntry = deserialize(&entry_bytes)?;
            last_hash = entry.chain_hash();
        }
        Ok(last_hash)
    }

    pub fn verify_full_chain(&self) -> Result<(), LedgerError> {
        let mut file = File::open(&self.path)?;
        let mut prev_hash_expected = [0u8; 32];
        let mut prev_roh = 0.0_f64;

        let mut buffer = [0u8; 4];
        while file.read_exact(&mut buffer).is_ok() {
            let len = u32::from_be_bytes(buffer) as usize;
            let mut entry_bytes = vec![0u8; len];
            file.read_exact(&mut entry_bytes)?;
            let entry: DonutLoopEntry = deserialize(&entry_bytes)?;

            // chain integrity
            if entry.prev_hash != prev_hash_expected {
                return Err(LedgerError::ChainBroken);
            }
            // signature
            let body = serialize(&(entry.prev_hash, entry.timestamp, entry.roh, &entry.tolerance, &entry.event, entry.pubkey))?;
            let sig = Signature::from_bytes(&entry.signature);
            let pubkey = PublicKey::from_bytes(&entry.pubkey).map_err(|_| LedgerError::BadSignature)?;
            pubkey.verify(&body, &sig).map_err(|_| LedgerError::BadSignature)?;

            // invariants
            if entry.roh > 0.3 || entry.roh < 0.0 {
                return Err(LedgerError::RoHExceeded);
            }
            if entry.roh > prev_roh {
                return Err(LedgerError::MonotoneViolation);
            }
            prev_roh = entry.roh;
            prev_hash_expected = entry.chain_hash();
        }
        Ok(())
    }
}

impl DonutLoopEntry {
    pub fn create(
        prev_hash: Hash,
        roh: f64,
        tolerance: ToleranceEnvelope,
        event: String,
        keypair: &Keypair,
    ) -> Result<Self, LedgerError> {
        if roh > 0.3 || roh < 0.0 {
            return Err(LedgerError::RoHExceeded);
        }
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let pubkey_bytes = keypair.public.to_bytes();
        let body = serialize(&(prev_hash, timestamp, roh, &tolerance, &event, pubkey_bytes))?;
        let signature = keypair.sign(&body);

        Ok(DonutLoopEntry {
            prev_hash,
            timestamp,
            roh,
            tolerance,
            event,
            pubkey: pubkey_bytes,
            signature: signature.to_bytes(),
        })
    }
}
