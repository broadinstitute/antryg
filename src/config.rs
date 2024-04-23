pub enum Config {
    Example,
    Mahal(MahalConfig),
}

pub struct MahalConfig {
    pub n_endos: usize,
    pub n_traits: usize,
    pub out: Option<String>
}