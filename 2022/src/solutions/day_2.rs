use crate::util::read_file;

pub fn part_one() -> String {
    let file = read_file(2);
    let mut score: u32 = 0;
    for battle in file.lines() {
        let opponent_shape = battle.chars().nth(0).unwrap().to_string();
        let own_shape = battle.chars().nth(2).unwrap().to_string();
        let mut win = 0;
        if (own_shape == "X" && opponent_shape == "A")
            || (own_shape == "Y" && opponent_shape == "B")
            || (own_shape == "Z" && opponent_shape == "C")
        {
            win = 3
        } else if (own_shape == "Y" && opponent_shape == "A")
            || (own_shape == "Z" && opponent_shape == "B")
            || (own_shape == "X" && opponent_shape == "C")
        {
            win = 6
        }
        score = score + get_value(own_shape.as_str()) + win
    }
    score.to_string()
}

pub fn part_two() -> String {
    let file = read_file(2);
    file.lines()
        .map(|line| {
            let opponent_choice = line.chars().nth(0).unwrap();
            let elf_hint = line.chars().nth(2).unwrap();
            get_score(opponent_choice, elf_hint)
        })
        .sum::<u32>()
        .to_string()
}

fn get_value(choice: &str) -> u32 {
    match choice {
        "X" | "A" => 1,
        "Y" | "B" => 2,
        _ => 3,
    }
}

fn get_score(opponent_choice: char, elf_hint: char) -> u32 {
    let opp_choice = opponent_choice.to_string();
    let own_choice;
    let score: u32 = match elf_hint.to_string().as_str() {
        "Y" => {
            own_choice = opp_choice.as_str();
            3
        }
        "X" => {
            own_choice = match opp_choice.as_str() {
                "A" => "C",
                "B" => "A",
                _ => "B",
            };
            0
        }
        _ => {
            own_choice = match opp_choice.as_str() {
                "A" => "B",
                "B" => "C",
                _ => "A",
            };
            6
        }
    };
    score + get_value(own_choice)
}
