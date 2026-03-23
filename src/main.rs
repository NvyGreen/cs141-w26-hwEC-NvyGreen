use std::sync::{Arc, RwLock, Mutex};
use std::fs::{File, OpenOptions};
use std::collections::HashMap;
use std::{thread, time};

struct Disk {
    sectors: [String; Disk::NUM_SECTORS]
}

impl Disk {
    const NUM_SECTORS: usize = 2048;
    const DISK_DELAY: u64 = 80;
    
    fn new() -> Self {
        Self { sectors: std::array::from_fn(|_| String::new()) }
    }

    fn write(&mut self, sector: usize, data: String) {
        thread::sleep(time::Duration::from_millis(Disk::DISK_DELAY));
        self.sectors[sector] = data.clone();
    }

    fn read(&self, sector: usize, data: &mut String) {
        thread::sleep(time::Duration::from_millis(Disk::DISK_DELAY));
        *data = self.sectors[sector].clone();
    }
}


struct Printer {
    out: File
}

impl Printer {
    const PRINT_DELAY: u64 = 275;

    fn new(id: usize) -> Self {
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


trait ResourceManager {
    fn request(&self) -> usize;
    fn release(&self, index: usize);
}


struct DiskManager {
    is_free: Arc<RwLock<Vec<bool>>>,
    disks: Vec<Disk>,
    next_free_sector: Vec<usize>,
    directory_manager: DirectoryManager
}


impl DiskManager {
    fn new(items: usize) -> Self {
        Self {
            is_free: Arc::new(RwLock::new(vec![true; items])),
            disks: std::iter::repeat_with(Disk::new).take(items).collect(),
            next_free_sector: vec![0; items],
            directory_manager: DirectoryManager::new()
        }
    }
}


struct PrinterManager {
    is_free: Arc<Mutex<Vec<bool>>>,
    printers: Vec<Printer>
}

impl PrinterManager {
    fn new(items: usize) -> Self {
        Self {
            is_free: Arc::new(Mutex::new(vec![true; items])),
            printers: (0..items).map(|i| Printer::new(i)).collect()
        }
    }
}


fn main() {
    println!("*** 141 OS Simulation ***");
}
