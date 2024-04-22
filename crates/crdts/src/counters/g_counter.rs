use super::Counter;

/// Grow-only counter implementation
#[derive(Debug, Clone)]
pub struct GCounter<T> {
    id: usize,
    entries: Vec<T>,
}

impl<T> GCounter<T> {
    pub fn new(id: usize, total_nodes: Option<u64>) -> GCounter<T> {
        let nodes_number: usize = total_nodes.unwrap_or(std::u64::MAX) as usize;
        GCounter {
            id,
            entries: Vec::with_capacity(nodes_number),
        }
    }
}

impl Counter<u64> for GCounter<u64> {
    fn increment(&mut self) {
        self.entries[self.id] += 1;
    }

    fn decrement(&mut self) {
        unimplemented!("grow only counters can't decrement")
    }

    fn entries(&self) -> Vec<u64> {
        // self.entries
        self.entries.clone()
    }

    fn value(&self) -> u64 {
        self.entries.iter().sum()
    }

    fn compare(&self, counter: Box<dyn Counter<u64>>) -> bool {
        for (i, entry) in self.entries.iter().enumerate() {
            if *entry != counter.entries()[i] {
                return false;
            }
        }
        true
    }

    fn merge(&mut self, counter: Box<dyn Counter<u64>>) {
        for (i, entry) in self.entries.iter_mut().enumerate() {
            *entry = std::cmp::max(*entry, counter.entries()[i])
        }
    }
}
