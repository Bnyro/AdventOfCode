use crate::util::read_file;

pub fn part_one() -> String {
    let contents = read_file(3);
    assert_eq!(get_char_value('a'), 1);
    assert_eq!(get_char_value('A'), 27);
    contents
        .lines()
        .map(|line| {
            let first_part = &line[0..line.len() / 2];
            let second_part = &line[line.len() / 2..line.len()];
            let mut common_char = 'a';
            'outer: for char in first_part.chars() {
                for sec_char in second_part.chars() {
                    if char == sec_char {
                        common_char = char;
                        break 'outer;
                    }
                }
            }
            get_char_value(common_char)
        })
        .sum::<u32>()
        .to_string()
}

fn get_char_value(character: char) -> u32 {
    let order = character as u32;
    if character.is_uppercase() {
        order - 38
    } else {
        order - 96
    }
}
