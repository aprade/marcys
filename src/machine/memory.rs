pub use sys_metrics::memory::{
    Memory,
    get_memory
};

pub fn new() -> Memory {
    get_memory().unwrap()
}
