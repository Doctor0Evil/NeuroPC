use lifeforce_guards::LifeforceEnvelope;
use crate::auth::{XRAction, RejectionReason};

pub struct LifeforceGuard {
    envelope: LifeforceEnvelope,
}

impl LifeforceGuard {
    pub fn from_policies_dir(dir: &std::path::Path) -> anyhow::Result<Self> {
        let path = dir.join("lifeforce.aln");
        let envelope = LifeforceEnvelope::from_path(path)?;
        Ok(Self { envelope })
    }

    pub fn check(
        &self,
        mode: &str,
        current_lifeforce: f32,
        action: &XRAction,
    ) -> Result<(), RejectionReason> {
        let delta = -action.lifeforcecost;
        if let Err(err) = self.envelope.check_action(mode, current_lifeforce, delta) {
            return Err(RejectionReason {
                code: "LIFEFORCE_ENVELOPE".into(),
                message: format!("lifeforce guard violation: {err}"),
            });
        }
        Ok(())
    }
}
