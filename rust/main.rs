mod day1;

use askama::Template;
use axum::{
    extract::Form,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use lazy_static::lazy_static;
use std::sync::Mutex;
use day1::SumOfDistancesResults;

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
#[template(path = "sum_of_distances.html")] // using the template in this path, relative
struct LocationPairs<'a> {
    location_columns: &'a String,
    sum_of_distances: &'a i32
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(show_location_pairs))
        .route("/sum_of_distances", post(sum_of_distances));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler to display tasks
async fn show_location_pairs() -> Html<String> {
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
async fn sum_of_distances(Form(input): Form<RequestLocationColumns>) -> Html<String> {
    let location_pairs = day1::parse_data_and_sort(&input.location_columns.clone());

    let results: SumOfDistancesResults = day1::sum_differences(location_pairs);
    Html(results.render().unwrap())
}

// Structure to receive form data
#[derive(serde::Deserialize)]
struct RequestLocationColumns {
    location_columns: String,
}
