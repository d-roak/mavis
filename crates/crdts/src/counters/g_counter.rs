use super::Counter;

/// Grow-only counter implementation
#[derive(Debug, Clone)]
pub struct GCounter<T> {
    id: usize,
    pub entries: Vec<T>,
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

macro_rules! impl_GCounter {
    ( $($type:ty),* $(,)? ) => {
        $(
            impl Counter<$type, GCounter<$type>> for GCounter<$type> {
                fn increment(&mut self) {
                    self.entries[self.id] += 1;
                }

                fn decrement(&mut self) {
                    unimplemented!("grow only counters can't decrement")
                }

                fn value(&self) -> $type {
                    self.entries.iter().sum()
                }

                fn compare(&self, counter: GCounter<$type>) -> bool {
                    for (i, entry) in self.entries.iter().enumerate() {
                        if *entry != counter.entries[i] {
                            return false;
                        }
                    }
                    true
                }

                fn merge(&mut self, counter: GCounter<$type>) {
                    for (i, entry) in self.entries.iter_mut().enumerate() {
                        *entry = std::cmp::max(*entry, counter.entries[i])
                    }
                }
            }
        )*
    };
}

impl_GCounter!(u8, u16, u32, u64, u128, usize);
