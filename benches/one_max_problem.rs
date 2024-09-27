use bitvec::{array::BitArray, bitarr};
use criterion::{criterion_group, criterion_main, Criterion};
use evolve::{agent::Agent, simulation::Simulation};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct OneMaxAgent {
    genome: BitArray,
}

impl OneMaxAgent {
    pub fn new() -> Self {
        let mut agent = OneMaxAgent {
            genome: bitarr![0; 64],
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

        Self { genome }
    }

    fn mutate(&mut self, mutation_chance: f64) {
        for mut bit in self.genome.iter_mut() {
            if rand::thread_rng().gen::<f64>() < mutation_chance {
                *bit ^= true
            }
        }
    }

    fn evaluate(&self) -> f64 {
        self.genome.iter_ones().count() as f64
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("one_max_problem", |b| {
        b.iter(|| {
            let mut simulation: Simulation<OneMaxAgent> = Simulation::new(1000, 0.05, 1e-2);

            simulation.run(50);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
