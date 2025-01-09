use std::time::{Duration, SystemTime};
use std::{env, fs};
use std::path::Path;
use std::error::Error;

use thousands::Separable;

#[derive(serde::Serialize)]
#[derive(Clone)]
pub struct NamedElapsed {
    pub name: String,
    pub elapsed: Option<Duration>
}

use crate::filters::{elapsed_as_micros, elapsed_as_millis, elapsed_as_nanos};

pub fn format_u32(n: &u32) -> ::askama::Result<String> {
    Ok(n.separate_with_commas().into())
}

pub fn print_total(title: String, total:u32, elapseds: Vec<NamedElapsed>) {
	println!("------------------------------------------------------------------");
	print!("{: >20}: {: >10} [raw: {: >10}]\n",title,format_u32(&total).unwrap(),total);
	println!("-- Timings: --");
    for e in elapseds {
        print!("     {: >20}: {: >2} ms [ {: >6} μs; {: >10} ns]\n",
        e.name,
		elapsed_as_millis(&e.elapsed).unwrap(), // ms
		elapsed_as_micros(&e.elapsed).unwrap(), // ms
		elapsed_as_nanos(&e.elapsed).unwrap()); // μs
    }
	println!("------------------------------------------------------------------");
}

// convert multi-line string into a two sorted vectors of integers 
pub fn split_lists(location_columns: &str) -> Result<(Vec<u32>,Vec<u32>), Box<dyn Error>> {
    // and collect them into a vector
    // split each line of the data into two integers separated by spaces and collect them into two separate vectors
    let (locations_a, locations_b):  (Vec<u32>, Vec<u32>) =
        // split the two columns of location numbers into lines with two integers per line separated by space
        location_columns.lines()
        // split each line using spaces as the delimiter 
        .map(|s| s.split_whitespace())
        // parse each pair of strings into a tuple of integers
        .map(|mut s| (
            s.next().unwrap().parse::<u32>().unwrap(), 
            s.next().unwrap().parse::<u32>().unwrap()))
        // convert the interable of tuples of two integers into two separate vectors of integers
        .unzip();

    // return the two vectors of integers
    Ok((locations_a.clone(), locations_b.clone()))
}

pub fn split_and_sort_lists(lists: &String) -> Result<(Vec<u32>, Vec<u32>, Option<Duration>, Option<Duration>, Option<Duration>, Option<Duration>), Box<dyn Error>> {
    let start: SystemTime = SystemTime::now();

	let (mut list1, mut list2) = split_lists(&lists).expect("failed to split lists");
    let split_elapsed: Option<Duration> = start.elapsed().ok();

    let start_sort1: SystemTime = SystemTime::now();
    list1.sort();
    let sort1_elapsed: Option<Duration> = start_sort1.elapsed().ok();

    let start_sort2: SystemTime = SystemTime::now();
    list2.sort();
    let sort2_elapsed: Option<Duration> = start_sort2.elapsed().ok();

    Ok((list1, list2, split_elapsed, sort1_elapsed, sort2_elapsed, start.elapsed().ok()))
}

pub fn arg_or_default_path(arg_num: usize, default_paths: Vec<String>) -> Result<String, Box<dyn Error>> {
    // println!("current working directory {}", env::current_dir().unwrap().to_str().unwrap());
	let mut paths = default_paths.clone();

    let path: &String;
    
    let args: Vec<String>  = env::args().skip(1).collect();

    let path_opt =args.get(arg_num-1);
    if path_opt.is_some() {
        path = path_opt.unwrap();
        if path.trim().len() > 0 {
            // if a data file path was given via a command-line argument, then use that only...and ignore default paths
            paths.push(path.to_string());
        }
    }

    for p in paths.iter_mut() {
		if !Path::new(p).exists() {
			continue;
		}
        // println!("data path found: {}\n", p);
		return Ok(p.clone());
	}
	
	return Err(format!("failed to find valid data file from list: {}", paths.join(",")).into());
}

pub fn arg_or_default_int(arg_num: usize, default_num: i32) -> Result<i32, Box<dyn Error>> {
    let mut num = default_num;
    let arg_str: &String;
    
    let args: Vec<String>  = env::args().skip(1).collect();

    let arg_opt =args.get(arg_num-1);
    if arg_opt.is_some() {
        arg_str = arg_opt.unwrap();
        if arg_str.trim().len() > 0 {
            num = arg_str.parse::<i32>().unwrap();
        }
    }
    Ok(num)
}

pub fn load_text_file(file_path: String) -> Result<String, Box<dyn Error>> {

    fs::exists(&file_path).expect(format!("load_lists: file does not exist: {}", &file_path).as_str());

	Ok(fs::read_to_string(&file_path).expect(format!("load_lists: failed to load file: {}", &file_path).as_str()))
}
