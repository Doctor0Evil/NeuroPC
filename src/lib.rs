#![forbid(unsafe_code)]

pub mod sovereignty {
    pub mod invariants;
    pub mod policy;
    pub mod consent;
    pub mod ota_io;
    pub mod audit;
}

pub mod ota {
    pub mod controller;
}

pub mod evolution {
    pub mod controller;
}
