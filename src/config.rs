pub enum Config {
    Example,
    Mahal(MahalConfig),
    Marge(MargeConfig),
}

pub struct MahalConfig {
    pub n_endos: usize,
    pub n_traits: usize,
    pub out: Option<String>
}

pub struct MargeConfig {
    pub n_endos: usize,
    pub n_traits: usize,
}