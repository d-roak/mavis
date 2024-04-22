trait Counter<T> {
    fn increment(&mut self);
    fn decrement(&mut self);
    fn entries(&self) -> Vec<T>;
    fn value(&self) -> T;
    fn compare(&self, counter: Box<dyn Counter<T>>) -> bool;
    fn merge(&mut self, counter: Box<dyn Counter<T>>);
}

pub mod g_counter;
