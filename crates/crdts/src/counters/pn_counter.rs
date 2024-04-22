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

macro_rules! impl_PNCounter {
    ( $($type:ty),* $(,)? ) => {
        $(
            impl Counter<$type, PNCounter<$type>> for PNCounter<$type> {
                fn increment(&mut self) {
                    self.increments[self.id] += 1;
                }

                fn decrement(&mut self) {
                    self.decrements[self.id] += 1;
                }

                fn value(&self) -> $type {
                    self.increments.iter().sum::<$type>() - self.decrements.iter().sum::<$type>()
                }

                fn compare(&self, counter: PNCounter<$type>) -> bool {
                    for (i, entry) in self.increments.iter().enumerate() {
                        if *entry != counter.increments[i] || self.decrements[i] != counter.decrements[i] {
                            return false;
                        }
                    }
                    true
                }

                fn merge(&mut self, counter: PNCounter<$type>) {
                    for (i, entry) in self.increments.iter_mut().enumerate() {
                        *entry = std::cmp::max(*entry, counter.increments[i]);
                        self.decrements[i] = std::cmp::max(self.decrements[i], counter.decrements[i]);
                    }
                }
            }
        )*
    };
}

impl_PNCounter!(u8, u16, u32, u64, u128, usize);
