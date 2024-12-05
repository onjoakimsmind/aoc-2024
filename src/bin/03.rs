use regex::Regex;

advent_of_code::solution!(3);

fn process_mul_expression(expression: &str, num_re: &Regex) -> Option<u32> {
    let numbers: Vec<u32> = num_re
        .find_iter(expression)
        .filter_map(|m| m.as_str().parse::<u32>().ok())
        .collect();

    if let [a, b] = numbers.as_slice() {
        Some(a * b)
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let num_re = Regex::new(r"\d{1,3}").unwrap();

    let sum: u32 = re
        .find_iter(input)
        .filter_map(|cap| process_mul_expression(cap.as_str(), &num_re))
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();
    let num_re = Regex::new(r"\d{1,3}").unwrap();

    let mut enabled = true;
    let mut first_mul_used = false;
    let mut sum = 0;

    for cap in re.find_iter(input) {
        match cap.as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            instruction if instruction.starts_with("mul(") => {
                if !first_mul_used {
                    first_mul_used = true;
                    if let Some(product) = process_mul_expression(instruction, &num_re) {
                        sum += product;
                    }
                } else if enabled {
                    if let Some(product) = process_mul_expression(instruction, &num_re) {
                        sum += product;
                    }
                }
            }
            _ => {}
        }
    }

    Some(sum)
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
