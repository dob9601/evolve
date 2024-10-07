use crate::agent::Agent;

pub struct HallOfFame<T: Agent> {
    /// An ordered vector of agents up to size max_agents, storing from the best performing agent to the worst.
    agents: Vec<T>,

    /// The maximum number of agents to store in the hall of fame
    max_agents: usize,
}

impl<T: Agent> HallOfFame<T> {
    pub fn new(max_agents: usize) -> Self {
        Self {
            agents: vec![],
            max_agents,
        }
    }

    pub fn store(&mut self, agent: T) {
        let new_agent_evaluation = agent.evaluate();
        // FIXME(dob9601): Could optimise using a binary heap

        let position = self
            .agents
            .binary_search_by(|item| new_agent_evaluation.total_cmp(&item.evaluate()))
            .unwrap_or_else(|e| e);

        self.agents.insert(position, agent);

        self.truncate();
    }

    fn truncate(&mut self) {
        self.agents.truncate(self.max_agents);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Default)]
    struct MockAgent(f64);

    impl Agent for MockAgent {
        fn crossover(&self, other: &Self) -> Self {
            todo!()
        }

        fn mutate(&mut self, mutation_chance: f64) {
            todo!()
        }

        fn evaluate(&self) -> f64 {
            self.0
        }
    }

    #[test]
    fn test_hof_store() {
        let mut hof = HallOfFame::new(5);

        hof.store(MockAgent(13 as f64));
        hof.store(MockAgent(9 as f64));
        hof.store(MockAgent(10 as f64));
        hof.store(MockAgent(7 as f64));
        hof.store(MockAgent(5 as f64));
        hof.store(MockAgent(12 as f64));
        hof.store(MockAgent(11 as f64));
        hof.store(MockAgent(8 as f64));
        hof.store(MockAgent(6 as f64));

        dbg!(hof.agents.clone());

        assert_eq!(hof.agents.len(), 5);

        for (index, agent) in hof.agents.iter().enumerate() {
            assert_eq!(agent.0, (13 - index) as f64);
        }
    }
}
