use crate::evolution_frame::{EvolutionFrame, EvolutionPlane, EvolutionScope};
use crate::eco::EcoBandProfile;
use crate::lifeforce::LifeforceBand;
use crate::tokens::SugarCredits;

pub trait NeuromorphicAdapter {
    fn adapter_id(&self) -> &'static str;
    fn plane(&self) -> EvolutionPlane;
    fn scope(&self) -> EvolutionScope;

    fn eco_profile(&self) -> EcoBandProfile;

    fn sugar_balance(&self) -> SugarCredits;

    fn on_event(&mut self, event: &AdapterEvent) -> Option<EvolutionFrame>;
}

pub enum AdapterEvent {
    InputSpike,
    HostTaskScheduled,
    HostFeedback(f32),
    TimeTick(u64),
}

pub struct AdapterRuntime<T: NeuromorphicAdapter> {
    adapter: T,
}

impl<T: NeuromorphicAdapter> AdapterRuntime<T> {
    pub fn step(
        &mut self,
        event: AdapterEvent,
        lifeforce: LifeforceBand,
    ) -> Option<EvolutionFrame> {
        if lifeforce.is_hard_stop() || self.adapter.sugar_balance().is_empty() {
            return None;
        }
        self.adapter.on_event(&event)
    }
}
