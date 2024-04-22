use super::Counter;

/// Positive-Negative counter implementation
#[derive(Debug, Clone)]
pub struct PNCounter<T> {
    id: usize,
    pub increments: Vec<T>,
    pub decrements: Vec<T>,
}

impl<T> PNCounter<T> {
    pub fn new(id: usize, total_nodes: Option<u64>) -> PNCounter<T> {
        let nodes_number: usize = total_nodes.unwrap_or(std::u64::MAX) as usize;
        PNCounter {
            id,
            increments: Vec::with_capacity(nodes_number),
            decrements: Vec::with_capacity(nodes_number),
        }
    }
}

impl Counter<u64, PNCounter<u64>> for PNCounter<u64> {
    fn increment(&mut self) {
        self.increments[self.id] += 1;
    }

    fn decrement(&mut self) {
        self.decrements[self.id] += 1;
    }

    fn value(&self) -> u64 {
        self.increments.iter().sum::<u64>() - self.decrements.iter().sum::<u64>()
    }

    fn compare(&self, counter: PNCounter<u64>) -> bool {
        for (i, entry) in self.increments.iter().enumerate() {
            if *entry != counter.increments[i] || self.decrements[i] != counter.decrements[i] {
                return false;
            }
        }
        true
    }

    fn merge(&mut self, counter: PNCounter<u64>) {
        for (i, entry) in self.increments.iter_mut().enumerate() {
            *entry = std::cmp::max(*entry, counter.increments[i]);
            self.decrements[i] = std::cmp::max(self.decrements[i], counter.decrements[i]);
        }
    }
}
