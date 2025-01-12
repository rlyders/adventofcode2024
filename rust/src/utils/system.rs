use std::env::consts::ARCH;
use std::env::consts::OS;
use std::fmt::{self, Debug};
use sysconf::pagesize;
use sysinfo::{Pid, System};
use rustc_version_runtime::version;

use crate::filters::rustc_version;

use super::system_windows_n_macos::get_os_mem_stats_windows_n_macos;

// #[cfg(target_os = "windows")]

const BYTES_PER_KB: usize = 1024;

pub fn print_mem_stats(title: String) {
    #[cfg(target_os = "windows")]
    super::system_windows_n_macos::print_mem_stats_windows(title);

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
    return get_os_mem_stats_windows_n_macos();

    #[cfg(target_os = "linux")]
    return "? RAM".to_string();

    #[cfg(target_os = "macos")]
    return get_os_mem_stats_windows_n_macos();
}

// TODO check if this works on Windows/Linux
pub fn get_os_info() -> String {
    
    let mut os = OS;
    if os == "macos" {
        os = "MacOS"
    }
    format!("rust v{} on {} {} {}", version().to_string(), os, get_system_info(), get_os_mem_stats())
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

pub fn get_system_info() -> String {
    let mut sys: System = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    format!("{} ({} {}) {} CPUs",
    System::os_version().unwrap_or("?".to_string()),
    System::name().unwrap_or("?".to_string()), 
    System::kernel_version().unwrap_or("?".to_string()),
    sys.cpus().len())
    // // Display system information:
    // println!("System name:             {:?}", System::name());
    // println!("System kernel version:   {:?}", System::kernel_version());
    // println!("System OS version:       {:?}", System::os_version());
    // println!("System host name:        {:?}", System::host_name());

    // // Number of CPUs:
    // println!("NB CPUs: {}", sys.cpus().len());
}
