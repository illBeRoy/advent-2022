use std::fs;
use std::path::Path;

const INPUT_DIR: &str = "./assets/inputs";

pub fn read_input(filename: &str) -> String {
    let path_to_input = Path::new(INPUT_DIR).join(filename);
    let contents = fs::read_to_string(path_to_input);

    contents.expect(format!("missing input file: {}", filename).as_str())
}
