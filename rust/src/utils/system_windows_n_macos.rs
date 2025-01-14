use memory_stats::memory_stats;
use std::process;
use sysinfo::System;

// #[cfg(target_os = "windows")]
pub fn get_os_mem_stats_windows_n_macos() -> String {
    use crate::utils::system::bytes_str;

    let mut sys: System = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    format!(
        "{} RAM ({} used)",
        bytes_str(sys.total_memory() as usize, None),
        bytes_str(sys.used_memory() as usize, None),
    )
}

// #[cfg(target_os = "windows")]
pub fn print_mem_stats_windows_n_macos(title: String) {
    use crate::utils::system::bytes_str;

    let mut sys: System = System::new_all();

    sys.refresh_all();

    if let Some(usage) = memory_stats() {
        #[cfg(target_os = "windows")]
        let resident_ram = bytes_str(usage.physical_mem, None);
        #[cfg(target_os = "windows")]
        let ram = bytes_str(usage.virtual_mem, None);

        #[cfg(target_os = "macos")]
        let ram = "?"; // on MacOS, I can not find correlation between usage.physical_mem here and what appears as "VSZ" in ps... makes no sense to me so I'm not showing on MacOS
        #[cfg(target_os = "macos")]
        let resident_ram: String = bytes_str(usage.physical_mem, None);

        println!(
            "# [PID {:6}] {:-18}: RAM: {:8} (resident: {:8} share: {:8} code: {:8} data: {:8})",
            process::id(),
            title,
            ram,
            resident_ram,
            "?", // bytes_str(s.share, None),
            "?", // bytes_str(s.text, None),
            "?", // bytes_str(s.data, None)
        );
    } else {
        println!("Couldn't get the current memory usage :(");
    }
}

pub fn print_system_cpu() {
    let mut sys: System = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // Display system information:
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());
}
