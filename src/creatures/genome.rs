#[derive(Debug, Clone, Copy)]
pub struct Genome {
    pub rest_length: f32,
    pub amplitude: f32,
    pub frequency: f32,
    pub phase_offset: f32,
}

impl Default for Genome {
    fn default() -> Self {
        Self {
            rest_length: 1.2,
            amplitude: 0.35,
            frequency: 1.2,
            phase_offset: 0.0,
        }
    }
}
