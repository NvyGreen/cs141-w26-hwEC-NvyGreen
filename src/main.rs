use std::sync::{Arc, RwLock};
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


struct DirectoryManager {
    t: HashMap<String, FileInfo>
}

impl DirectoryManager {
    fn new() -> Self {
        Self { t: HashMap::new() }
    }

    fn enter(&mut self, file_name: String, file: FileInfo) {
        self.t.insert(file_name, file);
    }

    fn lookup(&self, file_name: &str) -> Option<&FileInfo> {
        self.t.get(file_name)
    }
}


fn main() {
    println!("*** 141 OS Simulation ***");
}
