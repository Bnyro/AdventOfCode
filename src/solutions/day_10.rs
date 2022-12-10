use crate::util::read_file;

pub fn part_one() -> String {
    let input = read_file(10);
    let control_points: [i32; 6] = [20, 60, 100, 140, 180, 220];

    let mut current_cycle = 1;
    let mut current_value = 1;
    let mut sum = 0;

    for line in input.lines() {
        if control_points.contains(&current_cycle) {
            sum = sum + current_cycle * current_value;
        }
        if line == "noop" {
            current_cycle = current_cycle + 1;
            continue;
        }

        if control_points.contains(&(current_cycle + 1)) {
            sum = sum + (current_cycle + 1) * current_value;
        }
        let num = line.split_whitespace().last().unwrap().parse::<i32>().unwrap();
        current_value = current_value + num;
        current_cycle = current_cycle + 2;
    }
    sum.to_string()
}

pub fn part_two() -> String {
    "".to_string()
}