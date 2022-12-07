use crate::util::read_file;

pub fn part_one() -> String {
    solve(4)
}

pub fn part_two() -> String {
    solve(14)
}

fn solve(distinct_chars: usize) -> String {
    let chars: Vec<char> = read_file(6).chars().collect();
    let mut index: u32 = 0;
    let mut current_index = 0;
    while index == 0 {
        let mut four_chars: Vec<char> = vec![];
        for i in 0..distinct_chars {
            four_chars.push(chars[current_index + i]);
        }
        let mut cleaned_chars = four_chars.clone();
        cleaned_chars.sort();
        cleaned_chars.dedup();
        if cleaned_chars.len() == four_chars.len() {
            index = (current_index + distinct_chars) as u32;
            break;
        }
        current_index = current_index + 1;
    }
    index.to_string()
}
