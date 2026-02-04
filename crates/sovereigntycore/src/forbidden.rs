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
