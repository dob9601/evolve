use itertools::Itertools;
use log::info;
use rand::{thread_rng, Rng};
use statrs::statistics::Statistics;

use crate::{agent::Agent, stats::GenerationStatistics};

pub struct Simulation<T: Agent> {
    agents: Vec<T>,
    crossover_chance: f64,
    population_size: usize,
}

impl<T: Agent> Simulation<T> {
    pub fn new(population_size: usize, crossover_chance: f64) -> Self {
        Self {
            agents: (0..population_size).map(|_| T::default()).collect(),
            crossover_chance,
            population_size,
        }
    }

    pub fn run(&mut self, iterations: usize) {
        info!("Commencing evolutionary simulation");

        for i in 0..iterations {
            info!("Generation #{i}");

            let agents = std::mem::take(&mut self.agents);

            let agent_pairs = agents.into_iter().chunks(2);

            let next_generation = agent_pairs.into_iter().map(|chunk| {
                let mut chunk = chunk.collect_vec();
                if chunk.len() == 2 && thread_rng().gen::<f64>() < self.crossover_chance {
                    chunk[0].crossover(&chunk[1])
                } else {
                    std::mem::take(&mut chunk[0])
                }
            });

            let mut evaluated = next_generation
                .into_iter()
                .map(|agent| (agent.evaluate(), agent))
                .collect::<Vec<_>>();

            evaluated.sort_unstable_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap());

            self.agents = evaluated
                .into_iter()
                .map(|(_score, agent)| agent)
                .take(self.population_size)
                .collect()
        }
    }

    pub fn generate_stats(&self) -> GenerationStatistics {
        let scores = self
            .agents
            .iter()
            .map(|agent| agent.evaluate())
            .collect_vec();

        GenerationStatistics::new(scores.min(), scores.max(), scores.mean())
    }
}
