use std::fmt::Debug;

pub trait Agent: Default + Debug {
    fn crossover(&self, other: &Self) -> Self;

    fn mutate(&mut self);

    fn evaluate(&self) -> f64;
}
