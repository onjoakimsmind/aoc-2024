advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut a: Vec<u32> = vec![];
    let mut b: Vec<u32> = vec![];
    let rows: Vec<&str> = input.lines().collect();

    for row in rows {
        let parts: Vec<&str> = row.split_whitespace().collect();

        // Ensure the line has at least two parts
        if parts.len() >= 2 {
            if let Ok(num_a) = parts[0].parse::<u32>() {
                a.push(num_a);
            }
            if let Ok(num_b) = parts[1].parse::<u32>() {
                b.push(num_b);
            }
        }
    }
    a.sort();
    b.sort();
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
    None
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
