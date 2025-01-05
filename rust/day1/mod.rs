use askama::Template;

const DATA_FILE_PATH: &str = "../data/day1/lists.txt";

// Structure to send response of sorted lists and sum of distances
#[derive(Template)]
#[template(path = "results.html")] // using the template in this path, relative
pub struct SumOfDistancesResults {
    pub sorted_location_pairs: Vec<LocationPair>,
    pub sum_of_distances: u32
}

#[derive(serde::Serialize)]
struct LocationPair {
    location_a: u32,
    location_b: u32,
    distance: u32
}

pub fn main() {
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
        sum_of_distances: 0
    };
    for p in &results.sorted_location_pairs {
        results.sum_of_distances += p.distance;
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