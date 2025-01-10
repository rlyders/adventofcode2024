use procinfo::pid::statm_self;
use std::{
    fmt::{self, Debug},
    process,
};
use sysconf::pagesize;

const BYTES_PER_KB: usize = 1024;

use sysinfo::System;

pub fn print_mem_stats(title: String) {
    // let s = System::new_all();
    // println!("used: {}", s.used_memory() /1024 /1024);
    // println!("avail: {} kB", s.available_memory());
    // println!("CPU: {}", s.cpus().get(0).unwrap().brand());
    // println!("CPU usage: {}", s.global_cpu_usage());
    // println!("cores: {}", s.physical_core_count().unwrap());

    // details on values in statm: https://www.kernel.org/doc/Documentation/filesystems/proc.txt
    let r = statm_self();
    match r {
        Ok(s) => println!(
            "# [PID {:6}] {:-18}: RAM: {:8} (resident: {:8} share: {:8} code: {:8} data: {:8})",
            process::id(),
            title,
            pages_str(s.size),
            pages_str(s.resident),
            pages_str(s.share),
            pages_str(s.text),
            pages_str(s.data)
        ),
        Err(e) => eprintln!("failed to get results from statm: {}", e),
    }
}

pub fn pages_str(pages: usize) -> String {
    bytes_str(pages, Some(pagesize()))
}

#[derive(Debug, Clone, Copy)]
enum ByteUnit {
    B,
    KB,
    MB,
    GB,
    TB,
    PB,
}

impl fmt::Display for ByteUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ByteUnit::B => write!(f, "B"),
            ByteUnit::KB => write!(f, "KB"),
            ByteUnit::MB => write!(f, "MB"),
            ByteUnit::GB => write!(f, "GB"),
            ByteUnit::TB => write!(f, "TB"),
            ByteUnit::PB => write!(f, "PB"),
        }
    }
}

const BYTE_UNIT_CODES: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];

pub fn get_byte_units_code(byte_unit: ByteUnit) -> String {
    byte_unit.to_string()
}

pub fn bytes_str(bytes: usize, bytes_per_opt: Option<usize>) -> String {
    let bytes_per: usize = bytes_per_opt.unwrap_or(1024);
    let mut units_idx: usize = 0; // index to ram_units array
    let mut units: f32 = (bytes * bytes_per) as f32;
    while units > BYTES_PER_KB as f32 && units_idx < 4 {
        units /= BYTES_PER_KB as f32;
        units_idx += 1;
    }
    let s = format!("{:.1} {}", units, BYTE_UNIT_CODES.get(units_idx).unwrap());
    s
}
