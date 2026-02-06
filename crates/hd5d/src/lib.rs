use rand::Rng;

pub const DIM: usize = 10_000;

#[derive(Clone)]
pub struct Hypervector {
    pub bits: Vec<bool>,
}

impl Hypervector {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let bits = (0..DIM).map(|_| rng.gen_bool(0.5)).collect();
        Self { bits }
    }

    pub fn bind(&self, other: &Hypervector) -> Hypervector {
        let bits = self
            .bits
            .iter()
            .zip(other.bits.iter())
            .map(|(a, b)| *a ^ *b)
            .collect();
        Hypervector { bits }
    }

    pub fn similarity(&self, other: &Hypervector) -> f32 {
        let same = self
            .bits
            .iter()
            .zip(other.bits.iter())
            .filter(|(a, b)| a == b)
            .count() as f32;
        same / DIM as f32
    }
}

#[derive(Clone, Debug)]
pub struct Identity5D {
    pub biostate: String,
    pub neurostate: String,
    pub lifeforce: String,
    pub context: String,
    pub sovereignty: String,
}

pub struct IdentityEncoder {
    axis_base: Hypervector,
    bio_base: Hypervector,
    neuro_base: Hypervector,
    lifeforce_base: Hypervector,
    context_base: Hypervector,
    sovereignty_base: Hypervector,
}

impl IdentityEncoder {
    pub fn new() -> Self {
        Self {
            axis_base: Hypervector::random(),
            bio_base: Hypervector::random(),
            neuro_base: Hypervector::random(),
            lifeforce_base: Hypervector::random(),
            context_base: Hypervector::random(),
            sovereignty_base: Hypervector::random(),
        }
    }

    fn encode_label(&self, label: &str, seed: &Hypervector) -> Hypervector {
        let mut hv = seed.clone();
        for b in label.bytes() {
            let mut char_hv = Hypervector::random();
            for (i, bit) in char_hv.bits.iter_mut().step_by(257).enumerate() {
                if (b as usize + i) % 2 == 0 {
                    *bit = !*bit;
                }
            }
            hv = hv.bind(&char_hv);
        }
        hv
    }

    pub fn encode(&self, id: &Identity5D) -> Hypervector {
        let bio = self.encode_label(&id.biostate, &self.bio_base);
        let neuro = self.encode_label(&id.neurostate, &self.neuro_base);
        let life = self.encode_label(&id.lifeforce, &self.lifeforce_base);
        let ctx = self.encode_label(&id.context, &self.context_base);
        let sov = self.encode_label(&id.sovereignty, &self.sovereignty_base);

        self.axis_base
            .bind(&bio)
            .bind(&neuro)
            .bind(&life)
            .bind(&ctx)
            .bind(&sov)
    }
}
