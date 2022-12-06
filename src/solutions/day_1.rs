use crate::util::read_file;

pub fn solve() -> String {
    let content = read_file(1);
    let parts: Vec<&str> = content.split("\n\n").collect();
    let mut maximum_sum = 0;
    for i in 0..parts.len() {
        let sum = get_sum(parts[i]);
        if sum > maximum_sum {
            maximum_sum = sum;
        }
    }
    maximum_sum.to_string()
}

fn get_sum(part: &str) -> i32 {
    let mut sum = 0;
    let lines: Vec<&str> = part.lines().collect();
    for i in 0..lines.len() {
        sum = sum + lines[i].parse::<i32>().unwrap();
    }
    sum
}
