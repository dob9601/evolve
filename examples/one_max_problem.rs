use bitvec::{array::BitArray, bitarr};
use evolve::{agent::Agent, simulation::Simulation};
use log::info;
use rand::Rng;

#[derive(Debug)]
pub struct OneMaxAgent {
    genome: BitArray,
}

impl OneMaxAgent {
    pub fn new() -> Self {
        OneMaxAgent {
            genome: bitarr![0; 64],
        }
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

        Self { genome }
    }

    fn mutate(&mut self) {
        for mut bit in self.genome.iter_mut() {
            if rand::thread_rng().gen::<f32>() < 0.001 {
                *bit ^= true
            }
        }
    }

    fn evaluate(&self) -> f64 {
        (self.genome.iter_ones().count() as f64) / (self.genome.len() as f64)
    }
}

pub fn main() {
    env_logger::init();

    info!("Test");

    let mut simulation: Simulation<OneMaxAgent> = Simulation::new(2000, 0.05);

    simulation.run(20);
}
