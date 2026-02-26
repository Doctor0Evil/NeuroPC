#[derive(Debug, Clone)]
pub struct CopilotInput { /* fields mirroring ALN */ }

#[derive(Debug, Clone)]
pub struct CopilotOutput { /* fields mirroring ALN */ }

pub trait AdvisoryTool {
    fn propose(
        &self,
        input: &CopilotInput,
    ) -> Result<CopilotOutput, AdvisoryError>;
}
