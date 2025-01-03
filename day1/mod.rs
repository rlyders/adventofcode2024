pub fn main() {
    // load data/lists.tst file. There are two integers on each line. Split each row into two integers, and store them in two separate vectors.
    let (mut v1, mut v2): (Vec<i32>, Vec<i32>) = include_str!("./data/lists.txt")
        .lines()
        .map(|x| {
            let mut iter = x.split_whitespace();
            (
                iter.next().unwrap().parse::<i32>().unwrap(),
                iter.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();
    // sort the vectors in ascending order
    v1.sort();
    v2.sort();
    // loop over the two vectors and sum of the absolute differences between each pair of integers
    let mut sum = 0;
    for (a, b) in v1.iter().zip(v2.iter()) {
        sum += (a - b).abs();
    }
    println!("{}", sum);
}
