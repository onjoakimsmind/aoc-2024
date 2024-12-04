use advent_of_code::template::bootstrap;

advent_of_code::solution!(2);

fn is_safe(vec: &[u32]) -> bool {
    let is_increasing = vec.windows(2).all(|pair| pair[0] < pair[1]);
    let is_decreasing = vec.windows(2).all(|pair| pair[0] > pair[1]);
    let diff_within_three = vec
        .windows(2)
        .all(|pair| (pair[0] as i32 - pair[1] as i32).abs() <= 3);

    (is_increasing || is_decreasing) && diff_within_three
}

fn is_safe_with_dampener(vec: &[u32]) -> bool {
    if is_safe(vec) {
        return true;
    }

    for i in 0..vec.len() {
        let mut temp_vec = vec.to_vec();
        temp_vec.remove(i);
        if is_safe(&temp_vec) {
            return true;
        }
    }

    false
}

pub fn part_one(_input: &str) -> Option<u32> {
    let a = bootstrap::parse_to_vec1(_input);
    let mut sum = 0;

    for i in 0..a.len() {
        if is_safe(&a[i]) {
            sum += 1;
        }
    }

    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let a = bootstrap::parse_to_vec1(_input);
    let mut sum = 0;

    for i in 0..a.len() {
        if is_safe_with_dampener(&a[i]) {
            sum += 1;
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
