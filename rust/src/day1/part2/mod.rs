pub mod cmd;
pub mod web;
pub mod engine;

#[cfg(test)]
mod tests {
    use crate::utils::split_and_sort_lists;

    #[test]
    fn test_parse_data() {
        let lists = 
       "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";
        let (locations_a, locations_b, _elapsed) = split_and_sort_lists(&lists.to_string()).expect("failed to split and sort lists");
        assert_eq!(locations_a, vec![1,2,3,3,3,4]);
        assert_eq!(locations_b, vec![3,3,3,4,5,9]);
    }

    #[test]
    fn test_sum_differences() {
        let locations_a: Vec<u32> = vec![1,2,3,3,3,4];
        let locations_b: Vec<u32> = vec![3,3,3,4,5,9];
    
        let sum = super::engine::sum_similarity_scores(locations_a, locations_b).expect("failed to calculate similarity score");
        assert_eq!(sum.total_similarity_score, 31)
        }

}