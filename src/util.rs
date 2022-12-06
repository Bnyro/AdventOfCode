use std::fs;

pub fn read_file(day: i8) -> String {
    fs::read_to_string("inputs/day_".to_string() + day.to_string().as_str() + ".txt")
        .expect("Unable to read the file!")
}
