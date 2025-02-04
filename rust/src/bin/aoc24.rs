use aoc24lib::day1;
use aoc24lib::utils;
use aoc24lib::utils::system::get_os_info;
use aoc24lib::utils::system::print_mem_stats;

static DEFAULT_DATA_PATHS: &'static [&str] = &["./data", "../data"];

pub fn main() {
    println!("{}", get_os_info());
    print_mem_stats("START: main".to_string());
    let paths: Vec<String> = DEFAULT_DATA_PATHS.iter().map(|s| s.to_string()).collect();
    // println!("paths: {}", paths.join(","));

    day1::run(
        utils::arg_or_default_path(1, paths).unwrap(),
        utils::arg_or_default_int(2, 1).unwrap() as u32,
    );
    print_mem_stats("END  : main".to_string());
}
