pub trait Agent: Default {
    fn crossover(&self, other: &Self) -> Self;

    fn mutate(&mut self);

    fn evaluate(&self) -> f64;
}
