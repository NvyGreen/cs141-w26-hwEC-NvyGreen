use std::sync::{Arc, RwLock, Mutex, Condvar};
use std::fs::{File, OpenOptions};
use std::collections::HashMap;
use std::{thread, time};
use std::io::{Write, BufWriter, BufReader, BufRead};
use chrono::Local;

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
        *guard = data.clone();
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


struct PrintJobThread {
	file_name: String,
    file_info: FileInfo,
    disk_to_read: Arc<Disk>,
    printer_manager: Arc<PrinterManager>,
	log_file: Arc<Mutex<BufWriter<File>>>
}

impl PrintJobThread {
    fn new(file_name: String, file_info: FileInfo, disk_to_read: Arc<Disk>, printer_manager: Arc<PrinterManager>, log_file: Arc<Mutex<BufWriter<File>>>) -> Self {
        Self { file_name, file_info, disk_to_read, printer_manager, log_file }
    }

    fn run(self: Arc<Self>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let print_num = self.printer_manager.request();
            let mut printer = self.printer_manager.get_printer(print_num);
            let mut line = String::new();

			{
				let timestamp_start = Local::now();
				let mut log_start = self.log_file.lock().unwrap();
				writeln!(log_start, "Started printing file {} to PRINTER{} at {}", self.file_name, print_num, timestamp_start.format("%Y-%m-%dT%H:%M:%S")).unwrap();
				log_start.flush().unwrap();
			}

            for i in 0..self.file_info.file_length {
                self.disk_to_read.read(self.file_info.starting_sector + i, &mut line);
                printer.print(line.clone());
            }
            self.printer_manager.release(print_num);

			{
				let timestamp_end = Local::now();
				let mut log_end = self.log_file.lock().unwrap();
				writeln!(log_end, "Finished printing file {} to PRINTER{} at {}", self.file_name, print_num, timestamp_end.format("%Y-%m-%dT%H:%M:%S")).unwrap();
				log_end.flush().unwrap();
			}
        })
    }
}


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

	fn size(&self) -> usize {
		self.t.len()
	}
}


struct ResourceManager {
	is_free: Arc<(Mutex<Vec<bool>>, Condvar)>
}

impl ResourceManager {
	fn new(items: usize) -> Self {
		Self {
			is_free: Arc::new((
	            Mutex::new(vec![true; items]),
	            Condvar::new()
            )),
		}
	}
	
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


struct DiskManager {
    resource_manager: ResourceManager,
    disks: Vec<Arc<Disk>>,
    next_free_sector: Mutex<Vec<usize>>,
    directory_manager: Mutex<DirectoryManager>
}

impl DiskManager {
    fn new(items: usize) -> Self {
        Self {
            resource_manager: ResourceManager::new(items),
            disks: (0..items)
                .map(|_| Arc::new(Disk::new()))
                .collect(),
            next_free_sector: Mutex::new(vec![0; items]),
            directory_manager: Mutex::new(DirectoryManager::new())
        }
    }

	fn request(&self) -> usize {
		self.resource_manager.request()
	}

	fn release(&self, index: usize) {
		self.resource_manager.release(index);
	}

    fn get_disk(&self, index: usize) -> Arc<Disk> {
        Arc::clone(&self.disks[index])
    }

	fn get_num_files(&self) -> usize {
		self.directory_manager.lock().unwrap().size()
	}

    fn get_file_info(&self, file_name: String) -> Option<FileInfo> {
        let dm = self.directory_manager.lock().unwrap();
        dm.lookup(file_name)
    }

    fn get_next_sector(&self, index: usize) -> usize {
        self.next_free_sector.lock().unwrap()[index]
    }

	fn get_total_sectors_used(&self) -> usize {
		self.next_free_sector.lock().unwrap().iter().sum()
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


struct PrinterManager {
    resource_manager: ResourceManager,
    printers: Vec<Mutex<Printer>>
}

impl PrinterManager {
    fn new(items: usize) -> Self {
        Self {
            resource_manager: ResourceManager::new(items),
            printers: (0..items)
                .map(|i| Mutex::new(Printer::new(i)))
                .collect(),
        }
    }

	fn request(&self) -> usize {
		self.resource_manager.request()
	}

	fn release(&self, index: usize) {
		self.resource_manager.release(index);
	}

    fn get_printer(&self, index: usize) -> std::sync::MutexGuard<'_, Printer> {
        self.printers[index].lock().unwrap()
    }
}


struct UserThread {
    file_name: String,
    disk_manager: Arc<DiskManager>,
    printer_manager: Arc<PrinterManager>,
	log_file: Arc<Mutex<BufWriter<File>>>
}

impl UserThread {
    fn new(id: usize, disk_manager: Arc<DiskManager>, printer_manager: Arc<PrinterManager>, log_file: Arc<Mutex<BufWriter<File>>>) -> Self {
        Self {
            file_name: "users/USER".to_owned() + &id.to_string(),
            disk_manager,
            printer_manager,
			log_file
        }
    }

    fn run(self: Arc<Self>) -> thread::JoinHandle<Result<u64, Box<dyn std::error::Error + Send + Sync>>> {
        thread::spawn(move || -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
            let mut writing = false;
            let input_file = File::open(&self.file_name)?;
            let _in = BufReader::new(input_file);
            let mut print_jobs: Vec<thread::JoinHandle<()>> = Vec::new();

            let mut name_of_file = String::new();
            let mut sector = 2049;
            let mut start_sector = 2049;
            let mut disk_num = 27;
            let mut file_len = 0;
            let mut disk: Option<Arc<Disk>> = None;

            for line_result in _in.lines() {
                let line: String = line_result?;

                if line.starts_with(".save") && !writing {
                    let parts: Vec<&str> = line.split(" ").collect();
                    name_of_file = parts[1].to_string();

					if let Some(_) = self.disk_manager.get_file_info(name_of_file.clone()) {
						println!("Error: File {} already exists.", name_of_file);
					} else {
						disk_num = self.disk_manager.request();
	                    disk = Some(self.disk_manager.get_disk(disk_num));
	                    start_sector = self.disk_manager.get_next_sector(disk_num);
	                    sector = start_sector;
	                    writing = true;

						let timestamp = Local::now();
						let mut log = self.log_file.lock().unwrap();
						writeln!(log, "Started writing file {} to disk {} at {}", name_of_file, disk_num, timestamp.format("%Y-%m-%dT%H:%M:%S")).unwrap();
						log.flush().unwrap();
					}
                } else if line.starts_with(".end") && writing {
                    let info = FileInfo::new(disk_num, start_sector, file_len);
                    self.disk_manager.finish_disk(disk_num, sector, name_of_file.clone(), info);

					let timestamp = Local::now();
					let mut log = self.log_file.lock().unwrap();
					writeln!(log, "Finished writing file {} to disk {} at {}", name_of_file, disk_num, timestamp.format("%Y-%m-%dT%H:%M:%S")).unwrap();
					log.flush().unwrap();

                    name_of_file = String::new();
                    start_sector = 2049;
                    sector = 2049;
                    disk_num = 27;
                    file_len = 0;
                    disk = None;
                    writing = false;
                } else if line.starts_with(".print") && !writing {
                    let parts: Vec<&str> = line.split(" ").collect();
                    let print_out_file = parts[1].to_string();
                    
					if let Some(file_info) = self.disk_manager.get_file_info(print_out_file.clone()) {
						let read_disk = self.disk_manager.get_disk(file_info.disk_number);
	                    let job = Arc::new(PrintJobThread::new(print_out_file.clone(), file_info, read_disk, Arc::clone(&self.printer_manager), Arc::clone(&self.log_file)));
	                    print_jobs.push(job.run());
					} else {
						println!("Error: File {} not found.", print_out_file);
					}
                } else if writing {
                    disk.as_ref().unwrap().write(sector, line);
                    sector += 1;
                    file_len += 1;
                }
            }

			let num_print_jobs = print_jobs.len() as u64;
            for print_job in print_jobs {
                print_job.join().expect("A PrintJobThread panicked");
            }

            Ok(num_print_jobs)
        })
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    let num_users: usize = args[1][1..].parse().unwrap();
    let num_disks: usize = args[2][1..].parse().unwrap();
    let num_printers: usize = args[3][1..].parse().unwrap();
    
    println!("*** 141 OS Simulation ***");
	let timer = time::Instant::now();
	let log_file = Arc::new(Mutex::new(BufWriter::new(
		OpenOptions::new()
			.append(true)
			.create(true)
			.open("LOG")
			.unwrap()
	)));

    let disk_manager = Arc::new(DiskManager::new(num_disks));
    let printer_manager = Arc::new(PrinterManager::new(num_printers));

    let mut users: Vec<thread::JoinHandle<_>> = Vec::new();
    for i in 0..num_users {
        let user = Arc::new(UserThread::new(i, Arc::clone(&disk_manager), Arc::clone(&printer_manager), Arc::clone(&log_file)));
        users.push(user.run());
    }

	let mut total_print_jobs = 0;
    for user in users {
        if let Ok(Ok(count)) = user.join() {
			total_print_jobs += count;
		}
    }

	let duration = timer.elapsed();
	println!("\nRun finished. Summary:");
	println!("Total runtime: {:?}", duration);
	println!("Total files saved: {}", disk_manager.get_num_files());
	println!("Total print jobs: {}", total_print_jobs);
	println!("Total sectors used: {}", disk_manager.get_total_sectors_used());
}
