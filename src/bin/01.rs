use advent_of_code::template::bootstrap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (a, b) = bootstrap::parse_to_vec2(input);

    let mut sum = 0;

    for i in 0..a.len() {
        if a[i] < b[i] {
            sum += b[i] - a[i];
        } else {
            sum += a[i] - b[i];
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (a, b) = bootstrap::parse_to_vec2(input);
    let mut sum = 0;

    for i in 0..a.len() {
        sum += a[i] * b.iter().filter(|&n| *n == a[i]).count() as u32;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
