use std::fmt::Display;
use std::path::Path;

pub struct Stats {
    stats: Vec<GenerationStatistics>,
}

impl Stats {
    pub fn new() -> Self {
        Self { stats: vec![] }
    }

    pub fn push(&mut self, stat: GenerationStatistics) {
        self.stats.push(stat);
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

    /// Find the lowest scoring agent's score across any generation
    fn lowest_value(&self) -> f64 {
        self.stats
            .iter()
            .map(|stat| stat.min)
            .min_by(|a, b| a.total_cmp(&b))
            .unwrap()
    }

    fn highest_value(&self) -> f64 {
        self.stats
            .iter()
            .map(|stat| stat.max)
            .max_by(|a, b| a.total_cmp(&b))
            .unwrap()
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
