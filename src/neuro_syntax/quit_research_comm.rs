use super::smoking_cue_extinction::SmokingCueExtinctionWeight;

pub fn quit_research_comm(input: &str) -> String {
    let scew = SmokingCueExtinctionWeight::new(0.5, 0.4, 0.7, 0.5, 0.6);
    let sim = scew.compute_scew();
    format!("Research complete: SCEW={:.2}. Input: {} augmented safely.", sim, input)
}
