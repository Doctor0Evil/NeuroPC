use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// One line of the NDJSON manifest.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SovereignKernelItem {
    #[serde(rename = "riskmodel")]
    RiskModel(RiskModelSpec),

    #[serde(rename = "stakeschema")]
    StakeSchema(StakeSchemaSpec),

    #[serde(rename = "neurorightspolicy")]
    NeurorightsPolicy(NeurorightsPolicySpec),

    #[serde(rename = "tokenpolicy")]
    TokenPolicy(TokenPolicySpec),

    #[serde(rename = "evolvestreamspec")]
    EvolveStreamSpec(EvolveStreamSpec),

    #[serde(rename = "donutloopledgerspec")]
    DonutloopLedgerSpec(DonutloopLedgerSpec),

    #[serde(rename = "sovereigntyguardpipeline")]
    GuardPipeline(SovereigntyGuardPipelineSpec),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RiskModelAxis {
    pub name: String,
    pub weight: f32,
    pub min: f32,
    pub max: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RiskModelInvariants {
    pub rohceilingleq: f32,
    pub weightsnonnegative: bool,
    pub weightssumleq: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RiskModelSpec {
    pub id: String,
    pub subjectid: String,
    pub fileref: String,
    pub globalrohceiling: f32,
    pub axes: Vec<RiskModelAxis>,
    pub invariants: RiskModelInvariants,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StakeScopeSpec {
    pub scopeid: String,
    pub description: String,
    pub requiredroles: Vec<String>,
    pub tokenkindsallowed: Vec<String>,
    pub multisigrequired: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StakeRoleKindSpec {
    pub kind: String,
    pub minsigners: u8,
    pub maxsigners: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StakeSchemaInvariants {
    pub exactlyonehostpersubject: bool,
    pub lifeforceandarchrequiremultisig: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StakeSchemaSpec {
    pub id: String,
    pub subjectid: String,
    pub fileref: String,
    pub roles: Vec<StakeRoleKindSpec>,
    pub scopes: Vec<StakeScopeSpec>,
    pub invariants: StakeSchemaInvariants,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeurorightsPolicySpec {
    pub id: String,
    pub subjectid: String,
    pub fileref: String,
    pub dreamstate: NeurorightsDreamFlags,
    pub rights: NeurorightsCore,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeurorightsDreamFlags {
    pub dreamsensitive: bool,
    pub maxretentionhours: u32,
    pub storagescope: String,
    pub forbiddecisionuse: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeurorightsCore {
    pub mentalprivacy: bool,
    pub mentalintegrity: bool,
    pub cognitiveliberty: bool,
    pub noncommercialneuraldata: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenKindSpec {
    pub kind: String,
    pub description: String,
    pub maxeffectsizel2: f32,
    pub allowedscopes: Vec<String>,
    pub integrationdepth: String,
    pub requiresmultisig: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenPolicyEvCtrl {
    pub rohmonotonesafety: bool,
    pub rohafterleqbefore: bool,
    pub rohafterleqceiling: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenPolicySpec {
    pub id: String,
    pub subjectid: String,
    pub evctrl: TokenPolicyEvCtrl,
    pub tokens: Vec<TokenKindSpec>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvolveStreamSpec {
    pub id: String,
    pub subjectid: String,
    pub filepattern: String,
    pub recordtype: String,
    pub requiredfields: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DonutloopLedgerSpec {
    pub id: String,
    pub subjectid: String,
    pub fileref: String,
    pub decisionvalues: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuardStageSpec {
    pub order: u8,
    pub name: String,
    pub sourcecrate: String,
    pub function: String,
    pub failuredecision: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SovereigntyGuardPipelineSpec {
    pub id: String,
    pub subjectid: String,
    pub stages: Vec<GuardStageSpec>,
}

#[derive(Clone, Debug)]
pub struct SovereignKernelConfig {
    pub riskmodel: RiskModelSpec,
    pub stakeschema: StakeSchemaSpec,
    pub neurorights: NeurorightsPolicySpec,
    pub tokenpolicy: TokenPolicySpec,
    pub evolvestream: EvolveStreamSpec,
    pub donutloop: DonutloopLedgerSpec,
    pub guardpipeline: SovereigntyGuardPipelineSpec,
}

impl SovereignKernelConfig {
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("open manifest: {e}"))?;
        let reader = BufReader::new(file);

        let mut riskmodel: Option<RiskModelSpec> = None;
        let mut stakeschema: Option<StakeSchemaSpec> = None;
        let mut neurorights: Option<NeurorightsPolicySpec> = None;
        let mut tokenpolicy: Option<TokenPolicySpec> = None;
        let mut evolvestream: Option<EvolveStreamSpec> = None;
        let mut donutloop: Option<DonutloopLedgerSpec> = None;
        let mut guardpipeline: Option<SovereigntyGuardPipelineSpec> = None;

        for line in reader.lines() {
            let line = line.map_err(|e| format!("read line: {e}"))?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let item: SovereignKernelItem =
                serde_json::from_str(line).map_err(|e| format!("parse NDJSON: {e}"))?;
            match item {
                SovereignKernelItem::RiskModel(spec) => {
                    if riskmodel.is_some() {
                        return Err("duplicate riskmodel in manifest".into());
                    }
                    riskmodel = Some(spec);
                }
                SovereignKernelItem::StakeSchema(spec) => {
                    if stakeschema.is_some() {
                        return Err("duplicate stakeschema in manifest".into());
                    }
                    stakeschema = Some(spec);
                }
                SovereignKernelItem::NeurorightsPolicy(spec) => {
                    if neurorights.is_some() {
                        return Err("duplicate neurorightspolicy in manifest".into());
                    }
                    neurorights = Some(spec);
                }
                SovereignKernelItem::TokenPolicy(spec) => {
                    if tokenpolicy.is_some() {
                        return Err("duplicate tokenpolicy in manifest".into());
                    }
                    tokenpolicy = Some(spec);
                }
                SovereignKernelItem::EvolveStreamSpec(spec) => {
                    if evolvestream.is_some() {
                        return Err("duplicate evolvestreamspec in manifest".into());
                    }
                    evolvestream = Some(spec);
                }
                SovereignKernelItem::DonutloopLedgerSpec(spec) => {
                    if donutloop.is_some() {
                        return Err("duplicate donutloopledgerspec in manifest".into());
                    }
                    donutloop = Some(spec);
                }
                SovereignKernelItem::GuardPipeline(spec) => {
                    if guardpipeline.is_some() {
                        return Err("duplicate sovereigntyguardpipeline in manifest".into());
                    }
                    guardpipeline = Some(spec);
                }
            }
        }

        let cfg = SovereignKernelConfig {
            riskmodel: riskmodel.ok_or("missing riskmodel in manifest")?,
            stakeschema: stakeschema.ok_or("missing stakeschema in manifest")?,
            neurorights: neurorights.ok_or("missing neurorightspolicy in manifest")?,
            tokenpolicy: tokenpolicy.ok_or("missing tokenpolicy in manifest")?,
            evolvestream: evolvestream.ok_or("missing evolvestreamspec in manifest")?,
            donutloop: donutloop.ok_or("missing donutloopledgerspec in manifest")?,
            guardpipeline: guardpipeline.ok_or("missing sovereigntyguardpipeline in manifest")?,
        };

        cfg.validate()?;
        Ok(cfg)
    }

    pub fn validate(&self) -> Result<(), String> {
        // Risk model invariants.
        let inv = &self.riskmodel.invariants;
        if inv.rohceilingleq > self.riskmodel.globalrohceiling + 1e-6 {
            return Err("riskmodel invariant violated: rohceilingleq > globalrohceiling".into());
        }
        if inv.weightsnonnegative {
            if self
                .riskmodel
                .axes
                .iter()
                .any(|a| a.weight < 0.0 || !a.weight.is_finite())
            {
                return Err("riskmodel invariant violated: negative or non-finite weight".into());
            }
        }
        let wsum: f32 = self.riskmodel.axes.iter().map(|a| a.weight).sum();
        if wsum > inv.weightssumleq + 1e-6 {
            return Err("riskmodel invariant violated: weight sum exceeds limit".into());
        }

        // Stake schema invariants: one Host, lifeforce/arch scopes multisig.
        let host_roles: Vec<&StakeRoleKindSpec> = self
            .stakeschema
            .roles
            .iter()
            .filter(|r| r.kind.eq_ignore_ascii_case("Host"))
            .collect();
        if self.stakeschema.invariants.exactlyonehostpersubject && host_roles.len() != 1 {
            return Err("stakeschema invariant violated: expected exactly one Host role kind".into());
        }
        if self.stakeschema.invariants.lifeforceandarchrequiremultisig {
            for scope in &self.stakeschema.scopes {
                if scope.scopeid == "lifeforcealteration" || scope.scopeid == "archchange" {
                    if !scope.multisigrequired {
                        return Err(format!(
                            "stakeschema invariant violated: scope {} must require multisig",
                            scope.scopeid
                        ));
                    }
                    if !scope.tokenkindsallowed.iter().any(|k| k == "EVOLVE") {
                        return Err(format!(
                            "stakeschema invariant violated: scope {} must allow EVOLVE token",
                            scope.scopeid
                        ));
                    }
                }
            }
        }

        // Token policy invariants: RoH monotone, EVOLVE >= SMART.
        if !self.tokenpolicy.evctrl.rohmonotonesafety
            || !self.tokenpolicy.evctrl.rohafterleqbefore
        {
            return Err("tokenpolicy evctrl must enforce monotone RoH".into());
        }
        if (self.tokenpolicy.evctrl.rohafterleqceiling - self.riskmodel.globalrohceiling).abs()
            > 1e-6
        {
            return Err("tokenpolicy rohafterleqceiling must equal riskmodel.globalrohceiling"
                .into());
        }

        // Guard pipeline invariants: contiguous ordering, expected stages present.
        let mut orders: Vec<u8> = self.guardpipeline.stages.iter().map(|s| s.order).collect();
        orders.sort_unstable();
        for (i, o) in orders.iter().enumerate() {
            if *o as usize != i {
                return Err("guardpipeline orders must be contiguous from 0..n".into());
            }
        }
        let names: Vec<&str> = self
            .guardpipeline
            .stages
            .iter()
            .map(|s| s.name.as_str())
            .collect();
        for required in [
            "parseproposal",
            "loadpolicies",
            "rohguard",
            "neurorightsguard",
            "stakeguard",
            "tokenguard",
            "recorddecision",
        ] {
            if !names.iter().any(|n| *n == required) {
                return Err(format!(
                    "guardpipeline missing required stage {}",
                    required
                ));
            }
        }

        Ok(())
    }
}
