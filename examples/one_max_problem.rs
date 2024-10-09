use std::sync::{Arc, Mutex};

use bitvec::{array::BitArray, bitarr};
use evolve::{agent::Agent, simulator::MultithreadedSimulator};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct OneMaxAgent {
    genome: BitArray,
    evaluation: Arc<Mutex<Option<f64>>>,
}

impl OneMaxAgent {
    pub fn new() -> Self {
        let mut agent = OneMaxAgent {
            genome: bitarr![0; 64],
            evaluation: Arc::new(Mutex::new(None)),
        };

        agent.mutate(0.5);

        agent
    }
}

impl Default for OneMaxAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl Agent for OneMaxAgent {
    fn crossover(&self, other: &Self) -> Self {
        let mut genome = bitarr![0; 64];
        for (index, bit) in self.genome.iter().enumerate() {
            let other_bit = other.genome[index];

            if rand::thread_rng().gen() {
                genome.set(index, bit.as_ref().to_owned());
            } else {
                genome.set(index, other_bit);
            }
        }

        Self {
            genome,
            evaluation: Arc::new(Mutex::new(None)),
        }
    }

    fn mutate(&mut self, mutation_chance: f64) {
        for mut bit in self.genome.iter_mut() {
            if rand::thread_rng().gen::<f64>() < mutation_chance {
                *bit ^= true
            }
        }
    }

    fn evaluate_uncached(&self) -> f64 {
        let mut mutex = self.evaluation.lock().unwrap();

        if let Some(evaluation) = *mutex {
            evaluation
        } else {
            let evaluation = self.genome.iter_ones().count() as f64;

            *mutex = Some(evaluation);

            evaluation
        }
    }
}

pub fn main() {
    env_logger::init();

    let mut simulation: MultithreadedSimulator<OneMaxAgent> =
        MultithreadedSimulator::new(1000, 0.05, 1e-2);

    simulation.run(50);
}
