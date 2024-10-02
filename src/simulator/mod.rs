mod basic;
pub use basic::BasicSimulator;

mod multithreaded;
use log::{info, trace};
pub use multithreaded::MultithreadedSimulator;

use crate::agent::Agent;

pub trait Simulator<T: Agent> {
    fn simulate_generation(&mut self);

    fn generation_index(&self) -> usize;

    fn agents(&self) -> Vec<T>;

    fn run(&mut self, n_generations: usize) {
        info!(
            "Starting simulation at generation #{}",
            self.generation_index()
        );

        for _ in self.generation_index()..(self.generation_index() + n_generations) {
            trace!("Generation #{}", self.generation_index());
            self.simulate_generation();
        }

        info!(
            "Ending simulation at generation #{}",
            self.generation_index()
        )
    }

    fn best_agent(&self) -> Option<T> {
        self.agents()
            .iter()
            .max_by(|agent, other| agent.evaluate().total_cmp(&other.evaluate()))
            .cloned()
    }

    fn worst_agent(&self) -> Option<T> {
        self.agents()
            .iter()
            .min_by(|agent, other| other.evaluate().total_cmp(&agent.evaluate()))
            .cloned()
    }
}
