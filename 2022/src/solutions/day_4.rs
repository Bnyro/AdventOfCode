use crate::util::read_file;

pub fn part_one() -> String {
    let content = read_file(4);
    let mut count = 0;
    for line in content.lines() {
        let (first_range, second_range) = get_parts(line);
        if (first_range.0 >= second_range.0 && first_range.1 <= second_range.1)
            || (second_range.0 >= first_range.0 && second_range.1 <= first_range.1)
        {
            count = count + 1;
        }
    }
    count.to_string()
}

pub fn part_two() -> String {
    let content = read_file(4);
    let mut count = 0;
    for line in content.lines() {
        let (first_range, second_range) = get_parts(line);
        let first = first_range.0..first_range.1 + 1;
        let second = second_range.0..second_range.1 + 1;
        if !first
            .filter(|u| second.contains(&u))
            .collect::<Vec<u32>>()
            .is_empty()
        {
            count = count + 1;
        }
    }
    count.to_string()
}

fn get_parts(line: &str) -> ((u32, u32), (u32, u32)) {
    let pairs = line.split(",").collect::<Vec<&str>>();
    let first_range = get_range(pairs[0]);
    let second_range = get_range(pairs[1]);
    (first_range, second_range)
}

fn get_range(split: &str) -> (u32, u32) {
    let parts = split.split("-").collect::<Vec<&str>>();
    let min = parts[0].parse::<u32>().unwrap();
    let max = parts[1].parse::<u32>().unwrap();
    (min, max)
}
