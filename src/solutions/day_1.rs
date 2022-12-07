use crate::util::read_file;

pub fn part_one() -> String {
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

pub fn part_two() -> String {
    let mut elves = read_file(1)
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|str| str.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    elves.sort();
    elves.reverse();
    elves[0..3].to_vec().iter().sum::<i32>().to_string()
}
