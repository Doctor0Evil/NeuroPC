use crate::{BrainPrintRecord, BrainPrintExport};
use sovereignty_core::{SovereigntyCore, UpdateProposal, UpdateKind, UpdateEffectBounds};

pub struct BrainPrintExporter<'a, S, R>
where
    S: sovereignty_core::BiophysicalStateReader,
    R: BrainPrintRecord,
{
    sovereign: &'a mut SovereigntyCore<S>,
    _phantom: std::marker::PhantomData<R>,
}

impl<'a, S, R> BrainPrintExporter<'a, S, R>
where
    S: sovereignty_core::BiophysicalStateReader,
    R: BrainPrintRecord,
{
    pub fn new(sovereign: &'a mut SovereigntyCore<S>) -> Self {
        Self { sovereign, _phantom: std::marker::PhantomData }
    }

    pub fn export(&mut self, record: &R, evolvetoken_id: Option<&str>)
        -> (BrainPrintExport, sovereignty_core::AuditEntry)
    {
        let core   = record.core();
        let export = BrainPrintExport::from(core);

        // Treat this as a tiny, bounded ParamNudge in the audit trail.
        let proposal = UpdateProposal {
            id:          "brainprint-export".to_string(),
            module:      "brainprint.telemetry".to_string(),
            kind:        UpdateKind::ParamNudge,
            scope:       vec!["telemetry".to_string(), "brainprint".to_string()],
            description: "Emit exit-only brainPrint! summary".to_string(),
            effectbounds: UpdateEffectBounds {
                l2deltanorm: 0.0,
                irreversible: false,
            },
            requiresevolve: false,
        };

        let audit = self.sovereign.evaluate_update(proposal, evolvetoken_id);
        (export, audit)
    }
}
