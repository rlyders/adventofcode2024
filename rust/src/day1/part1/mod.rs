pub mod cmd;
pub mod engine;
pub mod web;

// function to load a list of strings into a single string from a given file in the data directory
pub fn load_data(file_path: &str) -> String {
    // read the file and return the contents
    return std::fs::read_to_string(file_path).expect("Error reading file");
}

