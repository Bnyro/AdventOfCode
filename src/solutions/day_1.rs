use crate::util::read_file;

pub fn solve() -> String {
    read_file(1)
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|str| str.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap()
        .to_string()
}
