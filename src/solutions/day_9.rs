use std::collections::BTreeSet;

use crate::util::read_file;

pub fn part_one() -> String {
    let input = read_file(9);
    let mut positions: BTreeSet<Position> = BTreeSet::new();
    let mut current_position = Position::default();
    for line in input.lines() {
        let action = line.chars().collect::<Vec<char>>()[0];
        let amount = line
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<i32>()
            .expect("Failed to parse amount");

        for i in 0..amount {
            match action {
                'U' => current_position.y = current_position.y + 1,
                'D' => current_position.y = current_position.y - 1,
                'R' => current_position.x = current_position.x + 1,
                'L' => current_position.x = current_position.x - 1,
                _ => {}
            }
            if i != amount - 1 {
                positions.insert(current_position.clone());
            }
        }
    }
    positions.len().to_string()
}

pub fn part_two() -> String {
    0.to_string()
}

#[derive(Ord, PartialEq, PartialOrd, Eq, Default, Clone)]
struct Position {
    x: i32,
    y: i32,
}
