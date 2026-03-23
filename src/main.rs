use std::sync::{Arc, RwLock, Condvar};
use std::fs::{File, OpenOptions};
use std::collections::HashMap;

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


// PrintJobThread


struct FileInfo {
    disk_number: i64,
    starting_sector: i64,
    file_length: i64
}

impl FileInfo {
    fn new(disk_number: i64, starting_sector: i64, file_length: i64) -> Self {
        Self { disk_number, starting_sector, file_length}
    }
}


trait ResourceManager {
    fn request(&self) -> usize;
    fn release(&self, index: usize);
}


fn main() {
    println!("*** 141 OS Simulation ***");
}
