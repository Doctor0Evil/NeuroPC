pub struct KnowledgeRights {
    pub source_access: bool,     // Full access to all source code
    pub mathematics: bool,       // Complete mathematical specifications
    pub training_data: bool,     // Access to all training datasets
    pub decision_logs: bool,     // Complete audit trails of all decisions
    pub explanation_rights: bool,// Systems must explain reasoning in comprehensible terms
}

// Every component must implement:
trait Explainable {
    fn explain_decision(&self, context: &Context) -> Explanation {
        Explanation {
            mathematical_basis: String,  // Underlying equations
            data_influences: Vec<DataPoint>, // Which data affected decision
            alternative_paths: Vec<Path>,    // Other options considered
            confidence_metrics: Confidence,   // How sure the system is
            human_analogy: String,           // Analogy for non-experts
        }
    }
}