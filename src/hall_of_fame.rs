use crate::agent::Agent;

pub struct HallOfFame<T: Agent> {
    /// An ordered vector of agents up to size max_agents.
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

    pub fn store(agent: T) {}
}
