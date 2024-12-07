use advent_of_code::template::bootstrap::parse_to_grid;
use std::collections::HashSet;

advent_of_code::solution!(6);
#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn find_arrow(grid: &Vec<Vec<char>>) -> Option<(usize, usize, char)> {
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &ch) in row.iter().enumerate() {
            if matches!(ch, '^' | '>' | 'V' | '<') {
                return Some((row_index, col_index, ch));
            }
        }
    }
    None
}

fn walk(
    direction: Direction,
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    positions: &mut HashSet<(usize, usize)>,
) {
    let (mut row, mut col) = start;

    // Add the starting position to the HashSet
    positions.insert((row, col));

    match direction {
        Direction::Up => {
            while row > 0 {
                if grid[row - 1][col] == '#' {
                    walk(Direction::Right, grid, (row, col), positions);
                    return;
                }
                row -= 1;
                positions.insert((row, col));
            }
        }
        Direction::Down => {
            while row + 1 < grid.len() {
                if grid[row + 1][col] == '#' {
                    walk(Direction::Left, grid, (row, col), positions);
                    return;
                }
                row += 1;
                positions.insert((row, col));
            }
        }
        Direction::Left => {
            while col > 0 {
                if grid[row][col - 1] == '#' {
                    walk(Direction::Up, grid, (row, col), positions);
                    return;
                }
                col -= 1;
                positions.insert((row, col));
            }
        }
        Direction::Right => {
            while col + 1 < grid[row].len() {
                if grid[row][col + 1] == '#' {
                    walk(Direction::Down, grid, (row, col), positions);
                    return;
                }
                col += 1;
                positions.insert((row, col));
            }
        }
    }
}

fn simulate_walk(
    direction: Direction,
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) {
    let (mut row, mut col) = start;
    let mut current_direction = direction;

    visited.insert((row, col));

    loop {
        match current_direction {
            Direction::Up => {
                if row > 0 && grid[row - 1][col] != '#' {
                    row -= 1;
                } else {
                    current_direction = Direction::Right;
                }
            }
            Direction::Down => {
                if row + 1 < grid.len() && grid[row + 1][col] != '#' {
                    row += 1;
                } else {
                    current_direction = Direction::Left;
                }
            }
            Direction::Left => {
                if col > 0 && grid[row][col - 1] != '#' {
                    col -= 1;
                } else {
                    current_direction = Direction::Up;
                }
            }
            Direction::Right => {
                if col + 1 < grid[row].len() && grid[row][col + 1] != '#' {
                    col += 1;
                } else {
                    current_direction = Direction::Down;
                }
            }
        }
        // if !visited.insert((row, col)) {
        //     break;
        // }

        if row == 0 || row == grid.len() - 1 || col == 0 || col == grid[row].len() - 1 {
            break;
        }
    }
}

pub fn part_one(_input: &str) -> Option<u32> {
    let grid = parse_to_grid(_input);
    let mut positions: HashSet<(usize, usize)> = HashSet::new();

    match find_arrow(&grid) {
        Some((row, col, ch)) => match ch {
            '^' => walk(Direction::Up, &grid, (row, col), &mut positions),
            '>' => walk(Direction::Right, &grid, (row, col), &mut positions),
            'V' => walk(Direction::Down, &grid, (row, col), &mut positions),
            '<' => walk(Direction::Left, &grid, (row, col), &mut positions),
            _ => (),
        },
        None => println!("No arrow characters found"),
    }

    println!("Total steps: {}", positions.len());
    Some(positions.len() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let grid = parse_to_grid(_input);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    match find_arrow(&grid) {
        Some((row, col, ch)) => match ch {
            '^' => simulate_walk(Direction::Up, &grid, (row, col), &mut visited),
            '>' => simulate_walk(Direction::Right, &grid, (row, col), &mut visited),
            'V' => simulate_walk(Direction::Down, &grid, (row, col), &mut visited),
            '<' => simulate_walk(Direction::Left, &grid, (row, col), &mut visited),
            _ => (),
        },
        None => println!("No arrow characters found"),
    }

    Some(visited.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }
}
