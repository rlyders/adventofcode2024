const DATA_FILE_PATH: &str = "../data/day1/lists.txt";

pub fn main() {
    // call the load_data function to load the data from the lists.txt file
    let data = load_data(DATA_FILE_PATH);
    let (x,y) = parse_data_and_sort(&data);

    let sum = sum_differences(x, y);
    // print the sum of the differences
    println!("Sum of differences in x: {}", sum);
}

pub fn sum_differences(x: Vec<i32>, y: Vec<i32>) -> i32 {
    // loop through the x vector and sum the differences between each element
    let mut sum = 0;
    for i in 0..x.len() {
        sum += (x[i] - y[i]).abs();
    }

    return sum;
}

// convert multi-line string into a two sorted vectors of integers 
pub fn parse_data_and_sort(data: &str) -> (Vec<i32>,Vec<i32>)  {
    // split the data into lines and collect them into a vector
    let data: Vec<String> = data.lines().map(|x| x.to_string()).collect();
    // split each line of the data into two integers separated by spaces and collect them into two separate vectors
    let (mut x, mut y): (Vec<i32>, Vec<i32>) = data.iter()
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .map(|x| (x[0].parse::<i32>().unwrap(), x[1].parse::<i32>().unwrap()))
        .unzip();

    // sort the two vectors
    x.sort();
    y.sort();

    return (x, y);
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
        let (x, y) = parse_data_and_sort(&lists);
        assert_eq!(x, vec![1,2,3,3,3,4]);
        assert_eq!(y, vec![3,3,3,4,5,9]);
    }

    #[test]
    fn test_sum_differences() {
        let x = vec![1,2,3,3,3,4];
        let y = vec![3,3,3,4,5,9];
    
        let sum = sum_differences(x, y);
        assert_eq!(sum, 11)
        }

}