use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// One row from policies/bostrom-stake-v1.stake.aln
/// CSV header (canonical):
/// roleid,subjectid,bostromaddress,rolekind,canveto,caninitevolve,requiredforlifeforce,requiredforarchchange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeRow {
    pub roleid: String,
    pub subjectid: String,
    pub bostromaddress: String,
    pub rolekind: String,
    pub canveto: bool,
    pub caninitevolve: bool,
    pub requiredforlifeforce: bool,
    pub requiredforarchchange: bool,
}

/// Logical scope of an action; use these labels from proposals/token scopes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeKind {
    Lifeforce,
    ArchChange,
    TsafePolicy,
    QPolicyUpdate,
    DreamRights,
    Other,
}

impl ScopeKind {
    pub fn from_str(s: &str) -> Self {
        match s {
            "lifeforce" => ScopeKind::Lifeforce,
            "archchange" => ScopeKind::ArchChange,
            "tsafe" | "tsafe_policy" => ScopeKind::TsafePolicy,
            "qpolicy" | "qpolicy_update" => ScopeKind::QPolicyUpdate,
            "dreamrights" => ScopeKind::DreamRights,
            _ => ScopeKind::Other,
        }
    }
}

/// Table of stake rows indexed by subject and by rolekind.
#[derive(Debug, Clone)]
pub struct StakeTable {
    /// subjectid -> Vec<StakeRow>
    by_subject: HashMap<String, Vec<StakeRow>>,
}

impl StakeTable {
    /// Load from canonical CSV-like stake file.
    /// Expected path: policies/bostrom-stake-v1.stake.aln
    pub fn load_from_file(path: &str) -> anyhow::Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut header = String::new();
        reader.read_line(&mut header)?;
        let header = header.trim();
        let expected = "roleid,subjectid,bostromaddress,rolekind,canveto,caninitevolve,requiredforlifeforce,requiredforarchchange";
        if header != expected {
            anyhow::bail!(
                "stake header mismatch: got '{}', expected '{}'",
                header,
                expected
            );
        }

        let mut by_subject: HashMap<String, Vec<StakeRow>> = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let cols: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
            if cols.len() != 8 {
                anyhow::bail!("invalid stake row, expected 8 columns: '{}'", line);
            }

            let row = StakeRow {
                roleid: cols[0].to_string(),
                subjectid: cols[1].to_string(),
                bostromaddress: cols[2].to_string(),
                rolekind: cols[3].to_string(),
                canveto: parse_bool(cols[4])?,
                caninitevolve: parse_bool(cols[5])?,
                requiredforlifeforce: parse_bool(cols[6])?,
                requiredforarchchange: parse_bool(cols[7])?,
            };

            by_subject
                .entry(row.subjectid.clone())
                .or_insert_with(Vec::new)
                .push(row);
        }

        let table = StakeTable { by_subject };
        table.validate_invariants()?;
        Ok(table)
    }

    /// Hard invariants:
    /// - exactly one Host role per subjectid
    /// - lifeforce and archchange scopes must have at least Host + OrganicCPU rows flagged as required
    fn validate_invariants(&self) -> anyhow::Result<()> {
        for (subject, rows) in &self.by_subject {
            let mut host_count = 0usize;
            let mut host_lifeforce_ok = false;
            let mut host_arch_ok = false;
            let mut organic_lifeforce_ok = false;
            let mut organic_arch_ok = false;

            for r in rows {
                if r.rolekind == "Host" {
                    host_count += 1;
                    if r.requiredforlifeforce {
                        host_lifeforce_ok = true;
                    }
                    if r.requiredforarchchange {
                        host_arch_ok = true;
                    }
                }
                if r.rolekind == "OrganicCPU" {
                    if r.requiredforlifeforce {
                        organic_lifeforce_ok = true;
                    }
                    if r.requiredforarchchange {
                        organic_arch_ok = true;
                    }
                }
            }

            if host_count != 1 {
                anyhow::bail!(
                    "stake invariant failed for subject {}: expected exactly 1 Host, found {}",
                    subject,
                    host_count
                );
            }

            if !host_lifeforce_ok || !organic_lifeforce_ok {
                anyhow::bail!(
                    "stake invariant failed for subject {}: lifeforce requires Host+OrganicCPU",
                    subject
                );
            }

            if !host_arch_ok || !organic_arch_ok {
                anyhow::bail!(
                    "stake invariant failed for subject {}: archchange requires Host+OrganicCPU",
                    subject
                );
            }
        }

        Ok(())
    }

    /// Return the set of required rolekinds for a given subject and action scope.
    pub fn required_roles_for_scope(
        &self,
        subjectid: &str,
        scope: ScopeKind,
    ) -> HashSet<String> {
        let mut required: HashSet<String> = HashSet::new();
        let rows = match self.by_subject.get(subjectid) {
            Some(r) => r,
            None => return required,
        };

        match scope {
            ScopeKind::Lifeforce => {
                for r in rows {
                    if r.requiredforlifeforce {
                        required.insert(r.rolekind.clone());
                    }
                }
            }
            ScopeKind::ArchChange => {
                for r in rows {
                    if r.requiredforarchchange {
                        required.insert(r.rolekind.clone());
                    }
                }
            }
            ScopeKind::TsafePolicy | ScopeKind::QPolicyUpdate | ScopeKind::DreamRights => {
                // Conservative default: require Host for all critical policy/dream scopes
                for r in rows {
                    if r.rolekind == "Host" {
                        required.insert(r.rolekind.clone());
                    }
                }
            }
            ScopeKind::Other => {
                // No hard requirement; may still be governed by SMART/EVOLVE tokens.
            }
        }

        required
    }

    /// Check that the provided signer (bostrom addresses) satisfy required roles
    /// for the given subject + scope. Returns Ok(()) if quorum is satisfied.
    pub fn check_multisig(
        &self,
        subjectid: &str,
        scope: ScopeKind,
        signer_addresses: &[String],
    ) -> anyhow::Result<()> {
        let required_roles = self.required_roles_for_scope(subjectid, scope);
        if required_roles.is_empty() {
            return Ok(());
        }

        let rows = self
            .by_subject
            .get(subjectid)
            .ok_or_else(|| anyhow::anyhow!("no stake rows for subject {}", subjectid))?;

        let signer_set: HashSet<String> =
            signer_addresses.iter().cloned().collect();

        let mut covered: HashSet<String> = HashSet::new();

        for r in rows {
            if !required_roles.contains(&r.rolekind) {
                continue;
            }
            if signer_set.contains(&r.bostromaddress) {
                covered.insert(r.rolekind.clone());
            }
        }

        if covered == required_roles {
            Ok(())
        } else {
            let missing: Vec<String> = required_roles
                .difference(&covered)
                .cloned()
                .collect();
            anyhow::bail!(
                "stake multisig violation: missing approvals from roles {:?} for subject {} scope {:?}",
                missing,
                subjectid,
                scope
            );
        }
    }
}

fn parse_bool(s: &str) -> anyhow::Result<bool> {
    match s {
        "true" | "True" | "1" => Ok(true),
        "false" | "False" | "0" => Ok(false),
        other => anyhow::bail!("invalid bool value '{}'", other),
    }
}
