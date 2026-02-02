use organiccpualn::stake::StakeShard;

pub struct StakeGate {
    shard: StakeShard,
    host_did: String,
}

impl StakeGate {
    pub fn new(shard: StakeShard, host_did: String) -> Self {
        Self { shard, host_did }
    }

    pub fn verify_host(&self) -> Result<(), String> {
        match self.shard.find_host_role(&self.host_did) {
            Some(_) => Ok(()),
            None => Err("No matching stakeholder row for host; automatic deny".to_string()),
        }
    }
}
