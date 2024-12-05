use advent_of_code::template::bootstrap;
use advent_of_code::template::bootstrap::Grid;

advent_of_code::solution!(4);

fn directions() -> Vec<(isize, isize)> {
    vec![
        (0, 1),   // Horizontal right
        (1, 0),   // Vertical down
        (1, 1),   // Diagonal down-right
        (1, -1),  // Diagonal down-left
        (0, -1),  // Horizontal left
        (-1, 0),  // Vertical up
        (-1, -1), // Diagonal up-left
        (-1, 1),  // Diagonal up-right
    ]
}

pub fn part_one(_input: &str) -> Option<u32> {
    let grid = bootstrap::parse_to_grid(_input);
    let word = "XMAS";
    let word_len = word.len();

    let mut sum = 0;

    let directions = directions();
    for (row, line) in grid.iter().enumerate() {
        for col in 0..line.len() {
            for &(dx, dy) in &directions {
                let mut found = true;
                for i in 0..word_len {
                    let nx = row as isize + dx * i as isize;
                    let ny = col as isize + dy * i as isize;
                    if nx < 0 || nx >= grid.len() as isize || ny < 0 || ny >= line.len() as isize {
                        found = false;
                        break;
                    }
                    if grid[nx as usize][ny as usize] != word.chars().nth(i).unwrap() {
                        found = false;
                        break;
                    }
                }
                if found {
                    sum += 1;
                }
            }
        }
    }
    print!("{}", sum);
    Some(sum)
}
pub fn part_two(_input: &str) -> Option<u32> {
    let grid: Grid<char> = _input.chars().collect();
    let mut sum = 0;

    for y in 0..grid.height as isize {
        for x in 0..grid.width as isize {
            if grid.get(x, y) != Some(&'A') {
                continue;
            }
            let Some(&ul) = grid.northwest(x, y) else {
                continue;
            };
            let Some(&ur) = grid.northeast(x, y) else {
                continue;
            };
            let Some(&ll) = grid.southwest(x, y) else {
                continue;
            };
            let Some(&lr) = grid.southeast(x, y) else {
                continue;
            };
            if ((ul == 'M' && lr == 'S') || (ul == 'S' && lr == 'M'))
                && ((ll == 'M' && ur == 'S') || (ll == 'S' && ur == 'M'))
            {
                sum += 1;
            }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
