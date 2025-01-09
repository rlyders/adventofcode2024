use crate::utils;

use lazy_static::lazy_static;
use std::{
    error::Error,
    iter,
    sync::Mutex,
    time::{Duration, SystemTime},
};

use super::web::{LocationPair, SumOfDistancesResults};

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

pub fn get_sum_of_distances_of_lists_text_repeats(
    lists_text: String,
    iterations: u32,
) -> Result<SumOfDistancesResults, Box<dyn Error>> {
	if iterations > 1 {
		println!("Iterations: {} ... all timings shown below are averages", iterations)
	}
    let mut results = SumOfDistancesResults {
        sorted_location_pairs: vec![],
        sum_of_distances: 0,
        elapseds: vec![],
    };
    let mut split_ns: u128 = 0;
    let mut sort1_ns: u128 = 0;
    let mut sort2_ns: u128 = 0;
    let mut split_and_sort_ns: u128 = 0;
    let mut calculate_ns: u128 = 0;
    let mut total_ns: u128 = 0;
    for _ in 1..=iterations {
        results = get_sum_of_distances_of_lists_text(&lists_text)?;

        for d in &results.elapseds {
            match d.name.as_str() {
                "split" => split_ns += d.elapsed.unwrap().as_nanos(),
                "sort1" => sort1_ns += d.elapsed.unwrap().as_nanos(),
                "sort2" => sort2_ns += d.elapsed.unwrap().as_nanos(),
                "split and sort" => split_and_sort_ns += d.elapsed.unwrap().as_nanos(),
                "calculate distance" => calculate_ns += d.elapsed.unwrap().as_nanos(),
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
            "calculate distance" => d.elapsed = Some(Duration::from_nanos(calculate_ns as u64/ iterations as u64)),
            "total" => d.elapsed = Some(Duration::from_nanos(total_ns as u64/ iterations as u64)),
            _ => panic!("unexpected elapsed name: {}", d.name.as_str()),
        }
    }

    Ok(results)
}

pub fn get_sum_of_distances_of_lists_text(
    lists_text: &String,
) -> Result<SumOfDistancesResults, Box<dyn Error>> {
    let start: SystemTime = SystemTime::now();

    let (sorted_list1, sorted_list2, split_elapsed, sort1_elapsed, sort2_elapsed, _) =
        utils::split_and_sort_lists(&lists_text).expect("failed to split and sort lists");
    let split_sort_elapsed = start.elapsed().ok();

    let mut results =
        get_sum_of_distances(sorted_list1, sorted_list2).expect("failed to calculate distances");

    results.elapseds.insert(
        0,
        utils::NamedElapsed {
            name: "split and sort".to_string(),
            elapsed: split_sort_elapsed,
        },
    );

    results.elapseds.insert(
        0,
        utils::NamedElapsed {
            name: "sort2".to_string(),
            elapsed: sort2_elapsed,
        },
    );

    results.elapseds.insert(
        0,
        utils::NamedElapsed {
            name: "sort1".to_string(),
            elapsed: sort1_elapsed,
        },
    );

    results.elapseds.insert(
        0,
        utils::NamedElapsed {
            name: "split".to_string(),
            elapsed: split_elapsed,
        },
    );

    results.elapseds.push(utils::NamedElapsed {
        name: "total".to_string(),
        elapsed: start.elapsed().ok(),
    });

    Ok(results)
}

pub fn get_sum_of_distances(
    sorted_list1: Vec<u32>,
    sorted_list2: Vec<u32>,
) -> Result<SumOfDistancesResults, Box<dyn Error>> {
    let start = SystemTime::now();
    // loop through the location pairs and sum the differences between each element
    let mut results = SumOfDistancesResults {
        sorted_location_pairs: Vec::with_capacity(sorted_list1.len()),
        sum_of_distances: 0,
        elapseds: vec![],
    };
    for (i, a) in sorted_list1.iter().enumerate() {
        let b = sorted_list2.get(i).unwrap();
        let location_pair = LocationPair {
            location_a: a.clone() as u32,
            location_b: b.clone() as u32,
            distance: a.abs_diff(*b),
        };
        results.sorted_location_pairs.push(location_pair.clone());
        results.sum_of_distances += location_pair.distance as u32;
    }
    results.elapseds.push(utils::NamedElapsed {
        name: "calculate distance".to_string(),
        elapsed: start.elapsed().ok(),
    });
    Ok(results)
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn test_parse_data() {
        let lists = "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";
        let locations = utils::split_and_sort_lists(&lists.to_string())
            .expect("failed to split and sort lists");

        let list1: Vec<u32> = vec![1, 2, 3, 3, 3, 4];
        let list2 = vec![3, 3, 3, 4, 5, 9];

        assert_eq!((locations.0, locations.1), (list1, list2));
    }

    #[test]
    fn test_sum_differences() {
        let list1: Vec<u32> = vec![1, 2, 3, 3, 3, 4];
        let list2 = vec![3, 3, 3, 4, 5, 9];

        let sum = get_sum_of_distances(list1, list2).expect("failed to get sum of distances");
        assert_eq!(sum.sum_of_distances, 11)
    }
}
