use std::fmt::Display;
use std::path::Path;

use statrs::statistics::Statistics;

pub struct Stats {
    stats: Vec<GenerationStatistics>,
}

impl Stats {
    pub fn new() -> Self {
        Self { stats: vec![] }
    }

    pub fn push(&mut self, scores: &[f64]) {
        let min = scores.min();
        let max = scores.max();
        let mean = scores.mean();
        let std_deviation = scores.std_dev();

        self.stats
            .push(GenerationStatistics::new(min, max, mean, std_deviation));
    }

    #[cfg(feature = "graphing")]
    pub fn save_graph<P: AsRef<Path>>(&self, path: &P) -> Result<(), Box<dyn std::error::Error>> {
        use plotters::prelude::*;

        let root = BitMapBackend::new(path, (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Covington | Output", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                0..self.stats.len(),
                self.lowest_value()..self.highest_value(),
            )?;

        chart.configure_mesh().draw()?;

        chart.draw_series(LineSeries::new(
            self.stats
                .iter()
                .enumerate()
                .map(|(index, stat)| (index, stat.min)),
            &RED,
        ))?;
        chart.draw_series(LineSeries::new(
            self.stats
                .iter()
                .enumerate()
                .map(|(index, stat)| (index, stat.max)),
            &GREEN,
        ))?;
        chart.draw_series(LineSeries::new(
            self.stats
                .iter()
                .enumerate()
                .map(|(index, stat)| (index, stat.mean)),
            &BLUE,
        ))?;
        chart.draw_series(LineSeries::new(
            self.stats
                .iter()
                .enumerate()
                .map(|(index, stat)| (index, stat.std_deviation)),
            &BLACK,
        ))?;

        chart.configure_series_labels().draw()?;

        root.present()?;
        Ok(())
    }

    /// Find the lowest scoring agent's score across any generation. Useful for scaling graphs.
    fn lowest_minima(&self) -> f64 {
        self.stats
            .iter()
            .map(|stat| stat.min)
            .min_by(|a, b| a.total_cmp(&b))
            .unwrap()
    }

    /// Find the highest scoring agent's score across any generation. Useful for scaling graphs.
    fn highest_maxima(&self) -> f64 {
        self.stats
            .iter()
            .map(|stat| stat.max)
            .max_by(|a, b| a.total_cmp(&b))
            .unwrap()
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

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
            "<Min: {:.3} | Max: {:.3} | Mean: {:.3} | Standard Deviation: {:.3}>",
            self.min, self.max, self.mean, self.std_deviation
        ))
    }
}
