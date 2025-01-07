use askama::Template;
use axum::{
    response::Html,
    routing::{get, post},
    Router,
};

mod day1_part_one;
mod day1_part_two;

use day1_part_one::{day1_part_one, sum_of_distances};
use day1_part_two::{day1_part_two, similarity_scores};

use serde_yaml; // 0.8.7

#[derive(serde::Deserialize, Template)]
#[template(path = "index.html")]
struct AdventOfCode {
    days: Vec<Day>,
}

#[derive(serde::Deserialize)]
struct Day {
    title: String,
    parts: Vec<Part>,
}

#[derive(serde::Deserialize)]
struct Part {
    name: String,
    summary: String,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // build our application with a route
    let app = Router::new()
        .route("/", get(home))
        .route("/day1/part1", get(day1_part_one))
        .route("/day1/part1/sum_of_distances", post(sum_of_distances))
        .route("/day1/part2", get(day1_part_two))
        .route("/day1/part2/similarity_scores", post(similarity_scores));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

pub async fn home() -> Html<String> {
    let f: std::fs::File = std::fs::File::open("../data/advent_of_code_2024.yaml").unwrap();
    let advent_of_code: AdventOfCode = serde_yaml::from_reader(f).unwrap();
    // println!("Read YAML string: {}", advent_of_code);

    Html(advent_of_code.render().unwrap())
}
