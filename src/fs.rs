use std::fs;

pub fn generate_file_name(name: &str) -> String {
    name
    .split("-")
    .map(|slice| {
        let (first_letter, rest) = slice.split_at(1);
        first_letter.to_uppercase() + rest
    })
    .collect() 
}

pub fn create_file(path: &str, content: String) {
    fs::write(&path, content.as_bytes()).expect("Error writing file");
}