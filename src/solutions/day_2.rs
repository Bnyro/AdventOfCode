use crate::util::read_file;

pub fn solve() -> String {
    let file = read_file(2);
    let battles: Vec<&str> = file.lines().collect();
    let mut score: i32 = 0;
    for i in 0..battles.len() {
        let opponent_shape = battles[i].chars().nth(0).unwrap().to_string();
        let own_shape = battles[i].chars().nth(2).unwrap().to_string();
        let value = match own_shape.as_str() {
            "X" => 1,
            "Y" => 2,
            _ => 3,
        };
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
        score = score + value + win
    }
    score.to_string()
}
