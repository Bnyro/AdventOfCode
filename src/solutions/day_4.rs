use crate::util::read_file;

pub fn part_one() -> String {
    let content = read_file(4);
    // assert_eq!(get_range("10-20"), 10..21);
    // assert!(get_range("10-20").contains(&10));
    let mut count = 0;
    for line in content.lines() {
        let pairs = line.split(",").collect::<Vec<&str>>();
        let first_range = get_range(pairs[0]);
        let second_range = get_range(pairs[1]);
        if (first_range.0 >= second_range.0 && first_range.1 <= second_range.1)
            || (second_range.0 >= first_range.0 && second_range.1 <= first_range.1)
        {
            count = count + 1;
        }
    }
    count.to_string()
}

fn get_range(split: &str) -> (u32, u32) {
    let parts = split.split("-").collect::<Vec<&str>>();
    let min = parts[0].parse::<u32>().unwrap();
    let max = parts[1].parse::<u32>().unwrap();
    (min, max)
}
