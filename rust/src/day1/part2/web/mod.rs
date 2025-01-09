use askama::Template;
use axum::{
    extract::Form,
    response::Html,
};
use super::engine::{LOCATION_COLUMNS};
use crate::{day1::part1::engine::get_sum_of_distances_of_lists_text_repeats, filters, utils::{self, split_and_sort_lists, NamedElapsed}};

// Structure to send response of sorted lists and sum of distances
#[derive(Template)]
#[template(path = "../src/day1/part2/web/templates/results.html")] // using the template in this path, relative
pub struct SimilarityResults {
    pub similarity_scores: Vec<SimilarityScore>,
    pub total_similarity_score: u32,
    pub elapseds: Vec<NamedElapsed>
}

#[derive(serde::Serialize)]
#[derive(Clone)]
pub struct SimilarityScore {
    pub location_a: u32,
    pub found_in_list_b: u32,
    pub similarity_score: u32
}

// Define the HTML template using Askama
#[derive(Template)]
#[template(path = "../src/day1/part2/web/templates/index.html")] // using the template in this path, relative
pub struct LocationPairs<'a> {
    pub location_columns: &'a String,
    pub sum_of_distances: &'a u32
}

// Handler to display tasks
#[axum::debug_handler]
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
    let results = get_sum_of_distances_of_lists_text_repeats(input.location_columns, input.iterations).expect("failed to calculate similarity score");
    Html(results.render().unwrap())    
}

// Structure to receive form data
#[derive(serde::Deserialize)]
pub struct RequestLocationColumns {
    location_columns: String,
    iterations: u32
}
