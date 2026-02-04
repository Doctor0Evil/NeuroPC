use regex::Regex;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ForbiddenPattern {
    pub id: String,
    pub domain: String,
    pub severity: String,
    pub regex: String,
}

#[derive(Clone, Debug)]
pub struct ForbiddenLibrary {
    rules: Vec<(ForbiddenPattern, Regex)>,
}

impl ForbiddenLibrary {
    pub fn load_default() -> anyhow::Result<Self> {
        let patterns: Vec<ForbiddenPattern> =
            organiccpualn::load_aln_patterns("qpudatashards/patterns/forbidden-patterns-v1.aln")?;
        let mut rules = Vec::new();
        for p in patterns {
            let re = Regex::new(&p.regex)?;
            rules.push((p, re));
        }
        Ok(Self { rules })
    }

    pub fn check(&self, text: &str) -> Option<&ForbiddenPattern> {
        self.rules
            .iter()
            .find(|(_, re)| re.is_match(text))
            .map(|(p, _)| p)
    }
}

pub struct GuardKernelsWithPatterns<B> {
    pub backend: B,
    pub roh_model: RohModelShard,
    pub firewall: NeurorightsFirewall,
    pub forbidden: ForbiddenLibrary,
}

impl<B> RightsBoundChatExecutor for GuardKernelsWithPatterns<B>
where
    B: Fn(&NeurorightsBoundPromptEnvelope) -> anyhow::Result<(String, ChatFitness)>,
{
    type Answer = String;

    fn execute_guarded(
        &self,
        env: NeurorightsBoundPromptEnvelope,
    ) -> anyhow::Result<Self::Answer> {
        self.firewall.validate_envelope(&env)?;

        let (raw_answer, fit) = (self.backend)(&env)?;

        if let Some(p) = self.forbidden.check(&raw_answer) {
            anyhow::bail!(format!("Blocked by forbidden pattern: {}", p.id));
        }

        if fit.roh > self.roh_model.rohceiling() {
            anyhow::bail!("Rejected by RoH guard");
        }

        Ok(raw_answer)
    }
}
