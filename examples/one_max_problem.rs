use bitvec::{array::BitArray, bitarr};
use evolve::agent::Agent;

struct OneMaxAgent {
    genome: BitArray,
}

impl OneMaxAgent {
    pub fn new() -> Self {
        OneMaxAgent {
            genome: bitarr![1; 64],
        }
    }
}

impl Agent for OneMaxAgent {
    fn mate(&self, other: &Self) -> Self {
        for (index, bit) in self.genome.iter().enumerate() {
            let other_bit = other.genome[index];
        }

        todo!();
    }

    fn mutate() -> Self {
        todo!()
    }
}

pub fn main() {}
