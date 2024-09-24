pub trait Agent {
    fn mate(&self, other: &Self) -> Self;

    fn mutate(&mut self);
}
