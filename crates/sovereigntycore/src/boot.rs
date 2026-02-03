use sovereignty_kernel_spec::SovereignKernelConfig;

pub struct SovereigntyCore {
    pub cfg: SovereignKernelConfig,
    // … existing fields: loaders, guards, corridor registry, etc.
}

impl SovereigntyCore {
    pub fn boot_for_subject(manifest_path: &str) -> Result<Self, String> {
        // 1. Load and validate sovereign kernel manifest.
        let cfg = SovereignKernelConfig::load_from_path(manifest_path)?;

        // 2. Initialize RoH, stake, neurorights, token guards from cfg.
        let roh_model = crate::riskofharm::load_model(&cfg.riskmodel)?;
        let stake_table = crate::stakeguard::load_stake(&cfg.stakeschema)?;
        let neurorights = crate::neurorights::load_policy(&cfg.neurorights)?;
        let token_policy = crate::tokenguard::load_policy(&cfg.tokenpolicy)?;
        let evolve_stream =
            crate::evolvestream::bind_stream(&cfg.evolvestream, &cfg.riskmodel, &cfg.tokenpolicy)?;
        let donutloop =
            crate::donutloop::bind_ledger(&cfg.donutloop, &cfg.tokenpolicy, &cfg.riskmodel)?;
        let guard_pipeline =
            crate::guards::build_pipeline(&cfg.guardpipeline, &roh_model, &neurorights,
                                          &stake_table, &token_policy, &donutloop)?;

        // 3. Only now construct SovereigntyCore and allow corridor / BrainSpecs to attach.
        Ok(SovereigntyCore {
            cfg,
            // … inject guard_pipeline, loaders, etc.
        })
    }
}
