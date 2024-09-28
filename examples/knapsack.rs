use itertools::Itertools;
use rand::Rng;

struct KnapsackItem {
    pub weight: u16,
    pub value: u16,
}

const ITEMS: [KnapsackItem; 10] = [
    KnapsackItem::new(10, 1),
    KnapsackItem::new(5, 5),
    KnapsackItem::new(10, 8),
    KnapsackItem::new(10, 3),
    KnapsackItem::new(30, 30),
    KnapsackItem::new(5, 8),
    KnapsackItem::new(2, 10),
    KnapsackItem::new(10, 1),
    KnapsackItem::new(17, 12),
    KnapsackItem::new(13, 9),
];

impl KnapsackItem {
    const fn new(weight: u16, value: u16) -> Self {
        Self { weight, value }
    }

    fn random() -> Self {
        let mut rng = rand::thread_rng();
        KnapsackItem::new(rng.gen_range(0..20), rng.gen_range(0..200))
    }
}

fn main() {}
