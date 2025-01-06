pub mod filters;

use askama::Template;
use axum::{
    extract::Form,
    response::Html,
    routing::{get, post},
    Router,
};
use lazy_static::lazy_static;
use std::{sync::Mutex, time::SystemTime};

const DATA_FILE_PATH: &str = "../data/day1/part1/lists.txt";

// Structure to send response of sorted lists and sum of distances
#[derive(Template)]
#[template(path = "day1/part1/results.html")] // using the template in this path, relative
pub struct SumOfDistancesResults {
    pub sorted_location_pairs: Vec<LocationPair>,
    pub sum_of_distances: u32,
    pub elapsed_ms: u32,
    pub elapsed_μs: u32
}

#[derive(serde::Serialize)]
struct LocationPair {
    location_a: u32,
    location_b: u32,
    distance: u32
}

pub fn cmd() {
    // call the load_data function to load the data from the lists.txt file
    let location_columns = load_data(DATA_FILE_PATH);
    let location_pairs = parse_data_and_sort(&location_columns);

    let sum = sum_differences(location_pairs);
    // print the sum of the differences
    println!("Sum of differences in x: {}", sum.sum_of_distances);
}

pub fn sum_differences(sorted_location_pairs: Vec<(i32, i32)>) -> SumOfDistancesResults {
    // loop through the location pairs and sum the differences between each element
    let mut results = SumOfDistancesResults {
        sorted_location_pairs: sorted_location_pairs.iter()
        .map(|(a,b)| LocationPair {
            location_a: a.clone() as u32,
            location_b: b.clone() as u32,
            distance: (a - b).unsigned_abs() as u32
        }).collect(),
        sum_of_distances: 0,
        elapsed_ms: 0,
        elapsed_μs: 0
    };
    for p in &results.sorted_location_pairs {
        results.sum_of_distances += p.distance as u32;
    }
    results
}

// convert multi-line string into a two sorted vectors of integers 
pub fn parse_data_and_sort(location_columns: &str) -> Vec<(i32,i32)> {
    // and collect them into a vector
    // split each line of the data into two integers separated by spaces and collect them into two separate vectors
    let (mut locations_a, mut locations_b):  (Vec<i32>, Vec<i32>) =
        // split the two columns of location numbers into lines with two integers per line separated by space
        location_columns.lines()
        // split each line using spaces as the delimiter 
        .map(|s| s.split_whitespace())
        // parse each pair of strings into a tuple of integers
        .map(|mut s| (
            s.next().unwrap().parse::<i32>().unwrap(), 
            s.next().unwrap().parse::<i32>().unwrap()))
        // convert the interable of tuples of two integers into two separate vectors of integers
        .unzip();

    // sort the two vectors of integers
    locations_a.sort();
    locations_b.sort();

    // return the two vectors of sorted integers as one vector of tuples
    locations_a.iter().zip(locations_b.iter()).map(|(a,b)| (a.clone(), b.clone())).collect()
}

// function to load a list of strings into a single string from a given file in the data directory
fn load_data(file_path: &str) -> String {
    // read the file and return the contents
    return std::fs::read_to_string(file_path).expect("Error reading file");
}

// Define a global shared state for storing location lists
lazy_static! {
    static ref LOCATION_COLUMNS: Mutex<String> = Mutex::new(String::from("3   4
4   3
2   5
1   3
3   9
3   3"));
}

// Define the HTML template using Askama
#[derive(Template)]
#[template(path = "day1/part1/index.html")] // using the template in this path, relative
struct LocationPairs<'a> {
    location_columns: &'a String,
    sum_of_distances: &'a u32
}

// Handler to display tasks
pub async fn day1_part_one() -> Html<String> {
    let location_columns = LOCATION_COLUMNS.lock().unwrap();
    let tot_sum_of_distances = 0;
    let template = LocationPairs {
        location_columns: &location_columns,
        sum_of_distances: &tot_sum_of_distances
    };
    Html(template.render().unwrap())
}

// create axum handler that accepts form data from a POST and returns a response of an integer
#[axum::debug_handler]
pub async fn sum_of_distances(Form(input): Form<RequestLocationColumns>) -> Html<String> {
    let now = SystemTime::now();

    let location_pairs = parse_data_and_sort(&input.location_columns.clone());

    let mut results: SumOfDistancesResults = sum_differences(location_pairs);

    match now.elapsed() {
        Ok(elapsed) => {
            results.elapsed_ms = elapsed.as_millis() as u32;
            println!("elapsed: {} ms", results.elapsed_ms);

            results.elapsed_μs = elapsed.as_micros() as u32;
            println!("elapsed: {} μs", results.elapsed_μs);
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {e:?}");
        }
    }
 
    Html(results.render().unwrap())    
}

// Structure to receive form data
#[derive(serde::Deserialize)]
pub struct RequestLocationColumns {
    location_columns: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data() {
        let lists = 
       "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";
        let locations = parse_data_and_sort(&lists);
        assert_eq!(locations, vec![(1,3),(2,3),(3,3),(3,4),(3,5),(4,9)]);
    }

    #[test]
    fn test_sum_differences() {
        let location_pairs = vec![(1,3),(2,3),(3,3),(3,4),(3,5),(4,9)];
    
        let sum = sum_differences(location_pairs);
        assert_eq!(sum.sum_of_distances, 11)
        }

}