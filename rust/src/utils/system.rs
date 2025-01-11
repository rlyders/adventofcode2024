use std::env::consts::ARCH;
use std::env::consts::OS;
use std::fmt::{self, Debug};
use sysconf::pagesize;

#[cfg(target_os = "windows")]
use crate::utils::system_windows::get_os_mem_stats_windows;

const BYTES_PER_KB: usize = 1024;

pub fn print_mem_stats(title: String) {
    #[cfg(target_os = "windows")]
    super::system_windows::print_mem_stats_windows(title);

    #[cfg(target_os = "linux")]
    super::system_linux::print_mem_stats_linux(title);
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
    let bytes_per: usize = bytes_per_opt.unwrap_or(1);
    let mut units_idx: usize = 0; // index to ram_units array
    let mut units: f32 = (bytes * bytes_per) as f32;
    while units > BYTES_PER_KB as f32 && units_idx < 4 {
        units /= BYTES_PER_KB as f32;
        units_idx += 1;
    }
    let s = format!("{:.1} {}", units, BYTE_UNIT_CODES.get(units_idx).unwrap());
    s
}

pub fn get_os_mem_stats() -> String {
    #[cfg(target_os = "windows")]
    return get_os_mem_stats_windows();

    #[cfg(target_os = "linux")]
    return "? RAM";
}

pub fn print_os_arch() {
    println!("{} {} {}", OS, ARCH, get_os_mem_stats());
    // hostStat, _ := host.Info()
    // cpuStat, _ := cpu.Info()
    // vmStat, _ := mem.VirtualMemory()
    // diskStat, _ := disk.Usage("\\") // If you're in Unix change this "\\" for "/"

    // info = new(SysInfo)

    // // info.Hostname = hostStat.Hostname
    // info.Platform = hostStat.Platform;
    // info.CPU = cpuStat[0].ModelName;
    // info.RAM = vmStat.Total / 1024 / 1024;
    // info.Disk = diskStat.Total / 1024 / 1024

    // fmt.Printf("OS: %s: ARCH: %s %s %s GB RAM\n", std::env::consts::OS, std::env::consts::ARCH, info.Platform, info.CPU, bytes_str(info.RAM*1024, None));
}
