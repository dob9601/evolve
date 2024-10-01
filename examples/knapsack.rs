use std::cmp::min;

use evolve::{agent::Agent, simulator::MultithreadedSimulator};
use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Clone, Debug)]
struct KnapsackItem {
    pub weight: i16,
    pub value: i16,
}

const ITEMS: [KnapsackItem; 10] = [
    KnapsackItem::new(10, 1),
    KnapsackItem::new(5, 5),
    KnapsackItem::new(10, 8),
    KnapsackItem::new(10, 3),
    KnapsackItem::new(30, 30),
    KnapsackItem::new(5, 8),
    KnapsackItem::new(2, 10),
    KnapsackItem::new(10, 1),
    KnapsackItem::new(17, 12),
    KnapsackItem::new(13, 9),
];

const MAX_WEIGHT: i16 = 30;
const MAX_ITEMS: i16 = 5;

#[derive(Clone, Debug)]
struct KnapsackAgent<'a> {
    sack: Vec<&'a KnapsackItem>,
}

impl KnapsackAgent<'_> {
    pub fn get_sack_weight(&self) -> i16 {
        self.sack.iter().map(|item| item.weight).sum()
    }

    pub fn get_sack_value(&self) -> i16 {
        self.sack.iter().map(|item| item.value).sum()
    }
}

impl Default for KnapsackAgent<'_> {
    fn default() -> Self {
        let mut rng = thread_rng();
        let mut sack = vec![];

        let total_weight = 0;

        while sack.len() < MAX_ITEMS as usize && total_weight < MAX_WEIGHT {
            sack.push(ITEMS.choose(&mut rng).unwrap());
        }

        KnapsackAgent { sack }
    }
}

impl Agent<i16> for KnapsackAgent<'_> {
    fn crossover(&self, other: &Self) -> Self {
        let mut rng = thread_rng();

        let min_length = min(other.sack.len(), self.sack.len());
        let crossover_point = rng.gen_range(0..min_length);

        let mut new_sack = self.sack[0..crossover_point].to_vec();
        new_sack.extend(other.sack[crossover_point..other.sack.len()].iter());

        Self { sack: new_sack }
    }

    fn mutate(&mut self, mutation_chance: f64) {
        let mut rng = thread_rng();

        let sack = std::mem::take(&mut self.sack);

        self.sack = sack
            .into_iter()
            .flat_map(|item| {
                let p = rng.gen::<f64>();

                if p < (mutation_chance / (1f64 / 3f64)) {
                    vec![ITEMS.choose(&mut rng).unwrap()] // Change the item
                } else if p < (mutation_chance / (2f64 / 3f64)) {
                    vec![] // Remove the item
                } else if p < mutation_chance {
                    vec![item, ITEMS.choose(&mut rng).unwrap()] // Keep the item, but add a new one
                } else {
                    vec![item] // Else, do nothing
                }
            })
            .collect_vec()
    }

    fn evaluate(&self) -> i16 {
        let sack_size_overflow = self.sack.len() as i16 - MAX_ITEMS as i16;
        let sack_weight_overflow = self.get_sack_weight() as i16 - MAX_WEIGHT as i16;

        let mut score = 0;

        if sack_size_overflow > 0 {
            score -= sack_size_overflow
        }

        if sack_weight_overflow > 0 {
            score -= sack_weight_overflow
        }

        if score != 0 {
            score.into()
        } else {
            self.get_sack_value().into()
        }
    }
}

impl KnapsackItem {
    const fn new(weight: i16, value: i16) -> Self {
        Self { weight, value }
    }
}

fn main() {
    env_logger::init();

    let mut simulator: MultithreadedSimulator<KnapsackAgent> =
        MultithreadedSimulator::new(10000, 1e-3, 1e-3);

    simulator.run(1000);
}
