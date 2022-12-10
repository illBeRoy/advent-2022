use std::fs;
use std::path::Path;

const INPUT_DIR: &str = "./assets/inputs";

pub fn input_for_day(day: u8) -> String {
    let filename = format!("day{}.txt", day);
    let path_to_input = Path::new(INPUT_DIR).join(&filename);
    let contents = fs::read_to_string(path_to_input);

    contents.expect(format!("missing input file: {}", filename).as_str())
}
