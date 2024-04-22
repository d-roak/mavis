/// Counter trait
/// T: Number to count
/// U: Type of counter
trait Counter<T, U> {
    fn increment(&mut self);
    fn decrement(&mut self);
    fn value(&self) -> T;
    fn compare(&self, counter: U) -> bool;
    fn merge(&mut self, counter: U);
}

pub mod g_counter;
pub mod pn_counter;
