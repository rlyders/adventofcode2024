use crate::{filters, utils::NamedElapsed};
use askama::Template;
use super::engine::*;

// Structure to send response of sorted lists and sum of distances
#[derive(Template)]
#[template(path = "../src/day1/part1/web/templates/results.html")] // using the template in this path, relative
pub struct SumOfDistancesResults {
    pub sorted_location_pairs: Vec<LocationPair>,
    pub sum_of_distances: u32,
    pub elapseds: Vec<NamedElapsed>,
}

#[derive(Clone, serde::Serialize)]
pub struct LocationPair {
    pub location_a: u32,
    pub location_b: u32,
    pub distance: u32,
}

// Define the HTML template using Askama
#[derive(Template)]
#[template(path = "../src/day1/part1/web/templates/index.html")] // using the template in this path, relative
pub struct LocationPairs<'a> {
    pub location_columns: &'a String,
    pub sum_of_distances: &'a u32,
}

use axum::{extract::Form, response::Html};

// create axum handler that accepts form data from a POST and returns a response of an integer
#[axum::debug_handler]
pub async fn sum_of_distances(Form(input): Form<RequestLocationColumns>) -> Html<String> {

    let results: SumOfDistancesResults = get_sum_of_distances_of_lists_text_repeats(input.location_columns, input.iterations).expect("failed to get sum of distances");

    Html(results.render().unwrap())
}

// Handler to display tasks
#[axum::debug_handler]
pub async fn day1_part_one() -> Html<String> {
    let location_columns = LOCATION_COLUMNS.lock().unwrap();
    let tot_sum_of_distances = 0;
    let template = LocationPairs {
        location_columns: &location_columns,
        sum_of_distances: &tot_sum_of_distances,
    };
    Html(template.render().unwrap())
}

// Structure to receive form data
#[derive(serde::Deserialize)]
pub struct RequestLocationColumns {
    location_columns: String,
    iterations: u32
}
