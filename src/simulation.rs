use log::info;

use crate::agent::Agent;

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

            let bred_population = agents.chunks(2).map(|chunk| {
                if chunk.len() < 2 {
                    return
                }
            })

            let mut next_generation = agents
                .into_iter()
                .map(|agent| (agent.evaluate(), agent))
                .collect::<Vec<_>>();

            next_generation.sort_unstable_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap());

            self.agents = next_generation
                .into_iter()
                .map(|(_score, agent)| agent)
                .take(self.population_size)
                .collect()
        }
    }
}
