pub struct Frac {
    numer: u64,
    denom: u64
}

impl From<u64> for Frac {
    fn from(numer: u64) -> Self {
        let denom = 1u64;
        Frac { numer, denom }
    }
}