use std::{error::Error, fs};

use crate::utils;
use super::{engine::get_sum_of_distances_of_lists_text_repeats, web::SumOfDistancesResults};

pub fn run(location_lists_path: String, iterations: u32) -> Result<(), Box<dyn Error>> {
    fs::exists(&location_lists_path).expect(format!("file does not exist: {}", location_lists_path).as_str());

	let lists: String = utils::load_text_file(location_lists_path).expect("failed to load location lists");

	let results: SumOfDistancesResults = get_sum_of_distances_of_lists_text_repeats(lists, iterations).expect("failed to get sum of distances");

	utils::print_total("Total Distance".to_string(), results.sum_of_distances, results.elapseds);
	
	Ok(())
}
