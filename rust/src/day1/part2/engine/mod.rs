use std::{error::Error, time::{Duration, SystemTime}};

use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::utils::{self, split_and_sort_lists};

use super::web::{SimilarityResults, SimilarityScore};

// Define a global shared state for storing location lists
lazy_static! {
    pub static ref LOCATION_COLUMNS: Mutex<String> = Mutex::new(String::from(
        "3   4
4   3
2   5
1   3
3   9
3   3"
    ));
}

pub fn calc_similarity(
    sorted_locations_a: Vec<u32>,
    sorted_locations_b: Vec<u32>,
) -> SimilarityResults {
    SimilarityResults {
        similarity_scores: sorted_locations_a
            .iter()
            .map(|&location_a| {
                let found_in_list_b = sorted_locations_b
                    .iter()
                    .filter(|&&location_b| location_b == location_a)
                    .count() as u32;
                SimilarityScore {
                    location_a,
                    found_in_list_b,
                    similarity_score: location_a * found_in_list_b,
                }
            })
            .collect(),
        total_similarity_score: 0,
        elapseds: vec![],
    }
}

pub fn calc_similarity_optimized(
    sorted_locations_a: Vec<u32>,
    sorted_locations_b: Vec<u32>,
) -> Vec<SimilarityScore> {
    let mut results: Vec<SimilarityScore> = vec![];
    let b_idx: &mut usize = &mut 0;
    let mut last_a: u32 = 0;
    let mut similarity_score: Option<SimilarityScore> = None;
    for (i, a) in sorted_locations_a.iter().enumerate() {
        // if this `a` value is *not* the same as the last
        // *or* this is first iteration of this loop
        if i == 0 || *a != last_a {
            last_a = *a;
            // find all locations in list B that match the location from list A
            similarity_score = Some(count_matching_values_in_list(
                *a,
                b_idx,
                &sorted_locations_b,
            ));
        }
        if similarity_score.is_some() {
            results.push(similarity_score.clone().unwrap());
        }
    }
    results
}

fn count_matching_values_in_list(
    src_val: u32,
    sorted_list_start_idx: &mut usize,
    sorted_list: &Vec<u32>,
) -> SimilarityScore {
    let mut similarity_score = SimilarityScore {
        location_a: src_val,
        found_in_list_b: 0,
        similarity_score: 0,
    };
    while *sorted_list_start_idx < sorted_list.len() {
        let b = sorted_list.get(*sorted_list_start_idx).unwrap();
        if src_val == *b {
            similarity_score.found_in_list_b += 1
        } else if *b > src_val {
            break; // the lists are both sorted, so if b is greater than a, then exit the loop since
                   // there won't be any more matches as the values will only get greater from this index
        }
        *sorted_list_start_idx += 1;
    }
    similarity_score.similarity_score = src_val * similarity_score.found_in_list_b;
    return similarity_score;
}

pub fn sum_similarity_scores(
    locations_a: Vec<u32>,
    locations_b: Vec<u32>,
) -> Result<SimilarityResults, Box<dyn Error>> {
    // loop through the location pairs and sum the differences between each element
    let start: SystemTime = SystemTime::now();

    let mut results = SimilarityResults {
        similarity_scores: calc_similarity_optimized(locations_a, locations_b),
        total_similarity_score: 0,
        elapseds: vec![],
    };
    for p in &results.similarity_scores {
        results.total_similarity_score += p.similarity_score as u32;
    }
    results.elapseds.push(utils::NamedElapsed {
        name: "calculate similarity".to_string(),
        elapsed: start.elapsed().ok(),
    });

    Ok(results)
}

pub fn similarity_scores_processor(lists_text: &String) -> Result<SimilarityResults, Box<dyn Error>>  {
    let start: SystemTime = SystemTime::now();

    let (locations_a, locations_b, split_elapsed, sort1_elapsed, sort2_elapsed, _) = split_and_sort_lists(lists_text).expect("failed to split and sort lists");
    let split_sort_elapsed = start.elapsed().ok();

    let mut results: SimilarityResults = sum_similarity_scores(locations_a, locations_b).expect("failed to calculate similarity score");

    results.elapseds.insert(0, utils::NamedElapsed { name: "split and sort".to_string(), elapsed: split_sort_elapsed});
    results.elapseds.insert(0, utils::NamedElapsed { name: "sort2".to_string(), elapsed: sort2_elapsed});
    results.elapseds.insert(0, utils::NamedElapsed { name: "sort1".to_string(), elapsed: sort1_elapsed});
    results.elapseds.insert(0, utils::NamedElapsed { name: "split".to_string(), elapsed: split_elapsed});
    results.elapseds.push(utils::NamedElapsed { name: "total".to_string(), elapsed: start.elapsed().ok()});
    
    Ok(results)
}

pub fn similarity_scores_processor_repeats(
    lists_text: &String,
    iterations: u32,
) -> Result<SimilarityResults, Box<dyn Error>> {
	if iterations > 1 {
		println!("Iterations: {} ... all timings shown below are averages", iterations)
	}
    let mut results = SimilarityResults {
        similarity_scores: vec![],
        total_similarity_score: 0,
        elapseds: vec![],
    };
    let mut split_ns: u128 = 0;
    let mut sort1_ns: u128 = 0;
    let mut sort2_ns: u128 = 0;
    let mut split_and_sort_ns: u128 = 0;
    let mut calculate_ns: u128 = 0;
    let mut total_ns: u128 = 0;
    for _ in 1..=iterations {
        results = similarity_scores_processor(lists_text)?;

        for d in &results.elapseds {
            match d.name.as_str() {
                "split" => split_ns += d.elapsed.unwrap().as_nanos(),
                "sort1" => sort1_ns += d.elapsed.unwrap().as_nanos(),
                "sort2" => sort2_ns += d.elapsed.unwrap().as_nanos(),
                "split and sort" => split_and_sort_ns += d.elapsed.unwrap().as_nanos(),
                "calculate similarity" => calculate_ns += d.elapsed.unwrap().as_nanos(),
                "total" => total_ns += d.elapsed.unwrap().as_nanos(),
                _ => panic!("unexpected elapsed name: {}", d.name.as_str()),
            }
        }
    }

    for d in &mut results.elapseds {
        match d.name.as_str() {
            "split" => d.elapsed = Some(Duration::from_nanos(split_ns as u64/ iterations as u64)),
            "sort1" => d.elapsed = Some(Duration::from_nanos(sort1_ns as u64/ iterations as u64)),
            "sort2" => d.elapsed = Some(Duration::from_nanos(sort2_ns as u64/ iterations as u64)),
            "split and sort" => d.elapsed = Some(Duration::from_nanos(split_and_sort_ns as u64/ iterations as u64)),
            "calculate similarity" => calculate_ns = calculate_ns / iterations as u128,
            "total" => total_ns = total_ns / iterations as u128,
            _ => panic!("unexpected elapsed name: {}", d.name.as_str()),
        }
    }

    Ok(results)
}
