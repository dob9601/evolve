pub struct GenerationStatistics {
    min: f64,
    max: f64,
    mean: f64,
}

impl GenerationStatistics {
    pub fn new(min: f64, max: f64, mean: f64) -> Self {
        Self { min, max, mean }
    }
}
