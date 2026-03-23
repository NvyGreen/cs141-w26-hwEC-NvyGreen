use std::sync::{Arc, RwLock, Mutex, Condvar};
use std::fs::{File, OpenOptions};
use std::collections::HashMap;
use std::{thread, time};
use std::io::{Write, BufWriter};

struct Disk {
    sectors: Vec<RwLock<String>>
}

impl Disk {
    const NUM_SECTORS: usize = 2048;
    const DISK_DELAY: u64 = 80;
    
    fn new() -> Self {
        Self {
            sectors: std::iter::repeat_with(|| RwLock::new(String::new()))
                .take(Self::NUM_SECTORS)
                .collect(),
        }
    }

    fn write(&self, sector: usize, data: String) {
        thread::sleep(time::Duration::from_millis(Disk::DISK_DELAY));
        let mut guard = self.sectors[sector].write().unwrap();
        *guard = data;
    }

    fn read(&self, sector: usize, data: &mut String) {
        thread::sleep(time::Duration::from_millis(Disk::DISK_DELAY));
        let guard = self.sectors[sector].read().unwrap();
        *data = guard.clone();
    }
}


struct Printer {
    out: BufWriter<File>
}

impl Printer {
    const PRINT_DELAY: u64 = 275;

    fn new(id: usize) -> Self {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("PRINTER".to_owned() + &id.to_string())
            .unwrap();
        Self { out: BufWriter::new(file) }
    }

    fn print(&mut self, data: String) {
        thread::sleep(time::Duration::from_millis(Printer::PRINT_DELAY));
        writeln!(self.out, "{}", data).unwrap();
        self.out.flush().unwrap();
    }
}


// PrintJobThread


#[derive(Clone)]
struct FileInfo {
    disk_number: usize,
    starting_sector: usize,
    file_length: usize
}

impl FileInfo {
    fn new(disk_number: usize, starting_sector: usize, file_length: usize) -> Self {
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

    fn lookup(&self, file_name: String) -> Option<FileInfo> {
        self.t.get(&file_name).cloned()
    }
}


trait ResourceManager {
    fn request(&self) -> usize;
    fn release(&self, index: usize);
}


struct DiskManager {
    is_free: Arc<(Mutex<Vec<bool>>, Condvar)>,
    disks: Vec<Arc<Disk>>,
    next_free_sector: Mutex<Vec<usize>>,
    directory_manager: Mutex<DirectoryManager>
}

impl DiskManager {
    fn new(items: usize) -> Self {
        Self {
            is_free: Arc::new((
                Mutex::new(vec![true; items]),
                Condvar::new()
            )),
            disks: (0..items)
                .map(|_| Arc::new(Disk::new()))
                .collect(),
            next_free_sector: Mutex::new(vec![0; items]),
            directory_manager: Mutex::new(DirectoryManager::new())
        }
    }

    fn get_disk(&self, index: usize) -> Arc<Disk> {
        Arc::clone(&self.disks[index])
    }

    fn get_file_info(&self, file_name: String) -> Option<FileInfo> {
        let dm = self.directory_manager.lock().unwrap();
        dm.lookup(file_name)
    }

    fn get_next_sector(&self, index: usize) -> usize {
        self.next_free_sector.lock().unwrap()[index]
    }

    fn finish_disk(&self, index: usize, new_free_sector: usize, file_name: String, info: FileInfo) {
        {
            let mut sectors = self.next_free_sector.lock().unwrap();
            sectors[index] = new_free_sector;
        }

        {
            let mut dm = self.directory_manager.lock().unwrap();
            dm.enter(file_name, info);
        }
        self.release(index);
    }
}

impl ResourceManager for DiskManager {
    fn request(&self) -> usize {
        let (lock, cvar) = &*self.is_free;
        let mut guard = lock.lock().unwrap();

        loop {
            for i in 0..guard.len() {
                if guard[i] {
                    guard[i] = false;
                    return i;
                }
            }

            guard = cvar.wait(guard).unwrap();
        }
    }

    fn release(&self, index: usize) {
        let (lock, cvar) = &*self.is_free;
        let mut guard = lock.lock().unwrap();
        guard[index] = true;
        cvar.notify_one();
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
