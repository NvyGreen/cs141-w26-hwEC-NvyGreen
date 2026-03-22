use std::sync::{Arc, RwLock};

struct Disk {
    sectors: [Arc<RwLock<String>>; Disk::NUM_SECTORS]
}

impl Disk {
    const NUM_SECTORS: usize = 2048;
    const DISK_DELAY: i64 = 80;
    
    fn new() -> Self {
        Self {
            sectors: std::array::from_fn(|_| {
                Arc::new(RwLock::new(String::new()))
            }),
        }
    }
}

fn main() {
    println!("*** 141 OS Simulation ***");
}
