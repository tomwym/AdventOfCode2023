use std::fs;

pub fn file_path(day: &str) -> String {
    String::from("dat/".to_owned() + day + ".dat")
}

pub fn input_as_vector_string(path: String, print: bool) -> Vec<String> {
    let file_content = fs::read_to_string(path)
        .expect("No file found: {e:?}");
    let as_vector: Vec<String> = file_content.split("\r\n").map(|v| v.to_string()).collect();
    if print {
        for val in &as_vector {
            println!("{val}");
        }
    }
    as_vector
}
