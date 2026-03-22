use std::sync::{Arc, RwLock};
use std::fs::{File, OpenOptions};

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


struct Printer {
    out: File
}

impl Printer {
    const PRINT_DELAY: i64 = 275;

    fn new(id: i64) -> Self {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("PRINTER".to_owned() + &id.to_string())
            .unwrap();
        Self { out: file }
    }
}


fn main() {
    println!("*** 141 OS Simulation ***");
}
