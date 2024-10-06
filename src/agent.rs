use std::fmt::Debug;

pub trait Agent: Default + Debug + Clone {
    /// Crossover, or breed, the agent with another one returning the new child agent.
    fn crossover(&self, other: &Self) -> Self;

    /// Mutate the agent
    fn mutate(&mut self, mutation_chance: f64);

    /// Evaluate the agent's fitness, ignoring any cached value.
    fn evaluate_uncached(&self) -> f64;

    /// Evaluate the agent, returning a cached value if there is one to avoid expensive function calls
    fn evaluate(&self) -> f64 {
        if let Some(evaluation) = self.get_cached_evaluation() {
            evaluation
        } else {
            let evaluation = self.evaluate();
            self.set_cached_evaluation(Some(evaluation));
            evaluation
        }
    }

    fn get_cached_evaluation(&self) -> Option<f64>;

    fn set_cached_evaluation(&self, evaluation: Option<f64>);
}
