#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SovereignIdentity {
    pub did: String,                    // Decentralized Identifier
    pub biometric_hash: [u8; 32],       // Zero-knowledge biometric proof
    pub consciousness_fingerprint: String, // EEG-derived unique pattern
    pub governance_shard: ShardReference, // ALN/DID governance rules
    pub consent_ledger: MerkleTree,     // Immutable consent records
}