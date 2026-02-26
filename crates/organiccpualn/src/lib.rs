pub trait AlnBackedProfile {
    fn load_from_aln(path: &str) -> Result<Self, Error>
    where
        Self: Sized;
    fn roh(&self) -> f32;
    fn is_actuation_free(&self) -> bool;
    fn is_monotone_tightening(&self, previous: &Self) -> bool;
}
