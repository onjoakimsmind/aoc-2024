use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let num = Regex::new(r"\d{1,3}").unwrap();
    let mut sum = 0;
    for mat in re.find_iter(input) {
        for m in num.find_iter(mat.as_str()) {
            println!("Found: {}", m.as_str());
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
