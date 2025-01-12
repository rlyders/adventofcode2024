use sysinfo::{Pid, System};
use memory_stats::memory_stats;

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
pub fn print_mem_stats_windows(title: String) {
    use crate::utils::system::bytes_str;

    let mut sys: System = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();
   
    // println!("=> system:");
    // RAM and swap information:
    if let Some(usage) = memory_stats() {
        println!(
            "{}: {} of {} RAM",
            title,
            bytes_str(usage.virtual_mem, None),
            bytes_str(usage.physical_mem, None),
            );
    } else {
        println!("Couldn't get the current memory usage :(");
    }

    // println!(
    //     "total swap  : {} bytes",
    //     bytes_str(sys.total_swap() as usize, None)
    // );
    // println!(
    //     "used swap   : {} bytes",
    //     bytes_str(sys.used_swap() as usize, None)
    // );

    // Display processes ID, name na disk usage:
    // for (pid, process) in sys.processes() {
    //     println!("[{pid}] {:?} {:?}", process.name(), process.disk_usage());
    // }

    // // We display all disks' information:
    // println!("=> disks:");
    // let disks = Disks::new_with_refreshed_list();
    // for disk in &disks {
    //     println!("{disk:?}");
    // }

    // // Network interfaces name, total data received and total data transmitted:
    // let networks = Networks::new_with_refreshed_list();
    // println!("=> networks:");
    // for (interface_name, data) in &networks {
    //     println!(
    //         "{interface_name}: {} B (down) / {} B (up)",
    //         data.total_received(),
    //         data.total_transmitted(),
    //     );
    //     // If you want the amount of data received/transmitted since last call
    //     // to `Networks::refresh`, use `received`/`transmitted`.
    // }

    // // Components temperature:
    // let components = Components::new_with_refreshed_list();
    // println!("=> components:");
    // for component in &components {
    //     println!("{component:?}");
    // }

    // match r {
    //     Ok(s) => println!(
    //         "# [PID {:6}] {:-18}: RAM: {:8} (resident: {:8} share: {:8} code: {:8} data: {:8})",
    //         process::id(),
    //         title,
    //         pages_str(s.size),
    //         pages_str(s.resident),
    //         pages_str(s.share),
    //         pages_str(s.text),
    //         pages_str(s.data)
    //     ),
    //     Err(e) => eprintln!("failed to get results from statm: {}", e),
    // }
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

// fn get_process_memory(pid: Pid) -> Result<u64, sysinfo::Error> {

//     let mut system = System::new();

//     system.refresh_process(pid)?;

//     let process = system.process(pid).unwrap();

//     Ok(process.memory())

// }

// Usage example:

// let process_memory = get_process_memory(1234).unwrap(); // Get memory usage of process with PID 1234

// println!("Process memory usage: {} bytes", process_memory);
