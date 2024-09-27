use std::time::Instant;

use itertools::Itertools;
use log::{info, trace};
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use statrs::statistics::Statistics;

use crate::{agent::Agent, stats::GenerationStatistics};

pub struct Simulation<T: Agent> {
    agents: Vec<T>,
    crossover_chance: f64,
    mutation_chance: f64,
    population_size: usize,
}

impl<T: Agent> Simulation<T> {
    pub fn new(population_size: usize, crossover_chance: f64, mutation_chance: f64) -> Self {
        Self {
            agents: (0..population_size).map(|_| T::default()).collect(),
            crossover_chance,
            population_size,
            mutation_chance,
        }
    }

    pub fn run(&mut self, generations: usize) {
        info!("Commencing simulation");

        for i in 0..generations {
            trace!("Starting generation #{i}");

            let now = Instant::now();

            let agents = std::mem::take(&mut self.agents);

            let agent_pairs = agents.into_iter().chunks(2);

            let next_generation = agent_pairs.into_iter().flat_map(|chunk| {
                let mut chunk = chunk.collect_vec();

                if chunk.len() == 2 && thread_rng().gen::<f64>() < self.crossover_chance {
                    let child = chunk[0].crossover(&chunk[1]);
                    chunk.push(child);
                }

                chunk
            });

            let evaluated = next_generation
                .map(|mut agent| {
                    agent.mutate(self.mutation_chance);
                    (agent.evaluate(), agent)
                })
                .sorted_unstable_by(|(a, _), (b, _)| b.total_cmp(a));

            self.agents = evaluated
                .map(|(_score, agent)| agent)
                .take(self.population_size)
                .collect();

            let elapsed_time = now.elapsed().as_millis() as f32 / 1000f32;

            info!(
                "Generation #{i} completed in {} seconds: {}",
                elapsed_time,
                self.generate_stats()
            );
        }
    }

    pub fn best_agent(&self) -> Option<T> {
        self.agents
            .iter()
            .max_by(|agent, other| agent.evaluate().total_cmp(&other.evaluate()))
            .cloned()
    }

    pub fn worst_agent(&self) -> Option<T> {
        self.agents
            .iter()
            .min_by(|agent, other| other.evaluate().total_cmp(&agent.evaluate()))
            .cloned()
    }

    pub fn generate_stats(&self) -> GenerationStatistics {
        let scores = self
            .agents
            .iter()
            .map(|agent| agent.evaluate())
            .collect_vec();

        GenerationStatistics::new(
            scores.clone().min(),
            scores.clone().max(),
            scores.clone().mean(),
            scores.std_dev(),
        )
    }
}
