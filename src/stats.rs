use std::fmt::Display;

pub struct GenerationStatistics {
    min: f64,
    max: f64,
    mean: f64,
    std_deviation: f64,
}

impl GenerationStatistics {
    pub fn new(min: f64, max: f64, mean: f64, std_deviation: f64) -> Self {
        Self {
            min,
            max,
            mean,
            std_deviation,
        }
    }
}

impl Display for GenerationStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "<Min: {} | Max: {} | Mean: {} | Standard Deviation: {}",
            self.min, self.max, self.mean, self.std_deviation
        ))
    }
}
