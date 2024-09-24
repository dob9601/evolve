pub trait Agent: Default {
    fn mate(&self, other: &Self) -> Self;

    fn mutate(&mut self);

    fn evaluate(&self) -> f64;
}
