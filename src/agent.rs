use std::fmt::Debug;

pub trait Agent: Default + Debug + Clone {
    /// Crossover, or breed, the agent with another one returning the new child agent.
    fn crossover(&self, other: &Self) -> Self;

    /// Mutate the agent
    fn mutate(&mut self, mutation_chance: f64);

    /// Evaluate the agent, returning a cached value if there is one to avoid expensive function calls
    fn evaluate(&self) -> f64 {
        if let Some(evaluation) = self.get_cached_evaluation() {
            evaluation
        } else {
            let evaluation = self.evaluate_uncached();
            self.set_cached_evaluation(Some(evaluation));

            evaluation
        }
    }

    fn evaluate_uncached(&self) -> f64;

    fn get_cached_evaluation(&self) -> Option<f64> {
        None
    }

    /// Set the value of the cached evaluation
    fn set_cached_evaluation(&self, evaluation: Option<f64>) {}
}
