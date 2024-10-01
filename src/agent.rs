use std::fmt::Debug;

pub trait Agent: Default + Debug + Clone {
    type EvaluationType;

    fn crossover(&self, other: &Self) -> Self;

    fn mutate(&mut self, mutation_chance: f64);

    fn evaluate(&self) -> Self::EvaluationType;
}
