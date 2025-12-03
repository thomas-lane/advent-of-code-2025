use std::fs;

pub fn input_to_string(input_name: &str) -> String {
    let file_path = format!("inputs/{}", input_name);
    let contents = fs::read_to_string(file_path).unwrap();

    contents
}