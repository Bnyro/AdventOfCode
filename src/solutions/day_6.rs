use std::collections::BTreeSet;

use crate::util::read_file;

pub fn part_one() -> String {
    solve(4)
}

pub fn part_two() -> String {
    solve(14)
}

fn solve(distinct_chars: usize) -> String {
    let chars: Vec<char> = read_file(6).chars().collect();
    let seq = chars
        .windows(distinct_chars)
        .enumerate()
        .find(|&(_index, c)| {
            let filtered = c.iter().collect::<BTreeSet<&char>>();
            filtered.len() == c.len()
        })
        .unwrap();

    (seq.0 + distinct_chars).to_string()
}
