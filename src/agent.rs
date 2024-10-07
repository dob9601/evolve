use std::fmt::Debug;

pub trait Agent: Default + Debug + Clone {
    /// Crossover, or breed, the agent with another one returning the new child agent.
    fn crossover(&self, other: &Self) -> Self;

    /// Mutate the agent
    fn mutate(&mut self, mutation_chance: f64);

    /// Evaluate the agent, returning a cached value if there is one to avoid expensive function calls
    fn evaluate(&self) -> f64;
}
