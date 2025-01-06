mod day1_part_one;

use day1_part_one::{load_data, parse_data_and_sort, sum_differences};

const DATA_FILE_PATH: &str = "../data/day1/lists.txt";

pub fn main() {
    // call the load_data function to load the data from the lists.txt file
    let location_columns = load_data(DATA_FILE_PATH);
    let location_pairs = parse_data_and_sort(&location_columns);

    let sum = sum_differences(location_pairs);
    // print the sum of the differences
    println!("Sum of differences in x: {}", sum.sum_of_distances);
}
