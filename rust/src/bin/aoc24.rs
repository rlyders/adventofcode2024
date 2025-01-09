use aoc24lib::day1;
use aoc24lib::utils;

static DEFAULT_DATA_PATHS: &'static [&str] = &["./data", "../data"];

pub fn main() {
    let paths: Vec<String> = DEFAULT_DATA_PATHS.iter().map(|s| s.to_string()).collect();
    // println!("paths: {}", paths.join(","));

    day1::run(
        utils::arg_or_default_path(1, paths).unwrap(),
        utils::arg_or_default_int(2, 1).unwrap() as u32,
    );
}
