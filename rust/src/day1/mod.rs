pub mod part1;
pub mod part2;

use std::path::Path;

use crate::utils::system::print_mem_stats;

pub fn run(data_path: String, iterations: u32 ) { 
    print_mem_stats("START: Day1".to_string());
    let path = Path::new(&data_path).join("day1").join("lists.txt");
	let location_lists_path = path.to_str().expect("failed to get path to lists.txt").to_string();

    part1::cmd::run(location_lists_path.clone(), iterations).expect("failed to run Day 1 Part One challenge");
    part2::cmd::run(location_lists_path, iterations).expect("failed to run Day 1 Part Two challenge");
    print_mem_stats("END  : Day1".to_string());
}
