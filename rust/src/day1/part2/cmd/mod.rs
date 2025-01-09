use std::{error::Error, fs, path::Path};

use crate::utils;

use super::engine::similarity_scores_processor_repeats;

pub fn run(location_lists_path: String, iterations: u32) -> Result<(), Box<dyn Error>> {
	let _ = Path::new(&location_lists_path).exists();

    fs::exists(&location_lists_path).expect(format!("file does not exist: {}", location_lists_path).as_str());

	let lists: String = utils::load_text_file(location_lists_path).expect("failed to load location lists");
	
	let results = similarity_scores_processor_repeats(&lists, iterations).expect("failed to get sum of distances of lists text");

	utils::print_total("Total Similarity".to_string(), results.total_similarity_score, results.elapseds);
	
	Ok(())
}
