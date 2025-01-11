#[cfg(target_os = "linux")]
use procinfo::pid::statm_self;

#[cfg(target_os = "linux")]
pub fn print_mem_stats_linux(title: String) {
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
