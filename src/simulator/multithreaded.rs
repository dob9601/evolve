use std::time::Instant;

use itertools::Itertools;
use log::{info, trace};
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use statrs::statistics::Statistics;

use crate::{
    agent::Agent,
    stats::{GenerationStatistics, Stats},
};

pub struct MultithreadedSimulator<T: Agent> {
    agents: Vec<T>,
    crossover_chance: f64,
    mutation_chance: f64,
    population_size: usize,
    generation: usize,
    pub stats: Stats,
}

impl<T: Agent + Send + Sync> MultithreadedSimulator<T> {
    pub fn new(population_size: usize, crossover_chance: f64, mutation_chance: f64) -> Self {
        Self {
            agents: (0..population_size).map(|_| T::default()).collect(),
            crossover_chance,
            population_size,
            mutation_chance,
            generation: 0,
            stats: Stats::new(),
        }
    }

    pub fn run(&mut self, generations: usize) {
        info!("Commencing simulation");

        for _ in 0..generations {
            trace!("Starting generation #{}", self.generation);

            let now = Instant::now();

            let agents = std::mem::take(&mut self.agents);

            let agent_pairs = agents.into_par_iter().chunks(2);

            let next_generation = agent_pairs.flat_map(|mut chunk| {
                if chunk.len() == 2 && thread_rng().gen::<f64>() < self.crossover_chance {
                    let child = chunk[0].crossover(&chunk[1]);
                    chunk.push(child);
                }

                chunk
            });

            let mut evaluated: Vec<_> = next_generation
                .map(|mut agent| {
                    agent.mutate(self.mutation_chance);
                    (agent.evaluate(), agent)
                })
                .collect();

            evaluated.par_sort_unstable_by(|(a, _), (b, _)| b.total_cmp(a));

            self.agents = evaluated
                .into_par_iter()
                .map(|(_score, agent)| agent)
                .take(self.population_size)
                .collect();

            let elapsed_time = now.elapsed().as_millis() as f32 / 1000f32;

            let generation_stats = self.generate_stats();

            self.generation += 1;
            info!(
                "Generation #{} completed in {} seconds: {}",
                self.generation, generation_stats, elapsed_time,
            );

            self.stats.push(
                &self
                    .agents
                    .iter()
                    .map(|agent| agent.evaluate())
                    .collect_vec(),
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
