pub mod filters;

use askama::Template;
use axum::{
    extract::Form,
    response::Html,
};
use lazy_static::lazy_static;
use std::{sync::Mutex, time::SystemTime};

// Structure to send response of sorted lists and sum of distances
#[derive(Template)]
#[template(path = "day1/part2/results.html")] // using the template in this path, relative
pub struct SimilarityResults {
    pub similarity_scores: Vec<SimilarityScore>,
    pub total_similarity_score: u32,
    pub elapsed_ms: u32,
    pub elapsed_μs: u32
}

#[derive(serde::Serialize)]
struct SimilarityScore {
    location_a: u32,
    found_in_list_b: u32,
    similarity_score: u32
}

pub fn sum_similarity_scores(locations_a: Vec<u32>, locations_b: Vec<u32>) -> SimilarityResults {
    // loop through the location pairs and sum the differences between each element
    let mut results = SimilarityResults {
        similarity_scores: locations_a.iter()
        .map(|&location_a| { 
            let found_in_list_b = locations_b.iter()
                .filter(|&&location_b| location_b == location_a).count() as u32;
            SimilarityScore{
            location_a,
            found_in_list_b,
            similarity_score: location_a * found_in_list_b }
        }).collect(),
        total_similarity_score: 0,
        elapsed_ms: 0,
        elapsed_μs: 0
    };
    for p in &results.similarity_scores {
        results.total_similarity_score += p.similarity_score as u32;
    }
    results
}

// convert multi-line string into a two sorted vectors of integers 
pub fn parse_data(location_columns: &str) -> (Vec<u32>,Vec<u32>) {
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
    (locations_a.clone(), locations_b.clone())
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
#[template(path = "day1/part2/index.html")] // using the template in this path, relative
struct LocationPairs<'a> {
    location_columns: &'a String,
    sum_of_distances: &'a u32
}

// Handler to display tasks
pub async fn day1_part_two() -> Html<String> {
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
pub async fn similarity_scores(Form(input): Form<RequestLocationColumns>) -> Html<String> {
    let now = SystemTime::now();

    let (locations_a, locations_b) = parse_data(&input.location_columns.clone());

    let mut results: SimilarityResults = sum_similarity_scores(locations_a, locations_b);

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
        let (locations_a, locations_b) = parse_data(&lists);
        assert_eq!(locations_a, vec![3,4,2,1,3,3]);
        assert_eq!(locations_b, vec![4,3,5,3,9,3]);
    }

    #[test]
    fn test_sum_differences() {
        let locations_a: Vec<u32> = vec![1,2,3,3,3,4];
        let locations_b: Vec<u32> = vec![3,3,3,4,5,9];
    
        let sum = sum_similarity_scores(locations_a, locations_b);
        assert_eq!(sum.total_similarity_score, 31)
        }

}