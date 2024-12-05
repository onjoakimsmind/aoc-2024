use std::collections::HashMap;

pub fn parse_to_vec(_input: &str) -> Vec<u32> {
    let rows: Vec<&str> = _input.lines().collect();
    rows.iter().filter_map(|&n| n.parse::<u32>().ok()).collect()
}

pub fn parse_to_vec1(input: &str) -> Vec<Vec<u32>> {
    let rows: Vec<&str> = input.lines().collect();

    rows.iter()
        .map(|row| {
            row.split_whitespace()
                .filter_map(|num| num.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

pub fn parse_to_vec2(_input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut a: Vec<u32> = vec![];
    let mut b: Vec<u32> = vec![];
    let rows: Vec<&str> = _input.lines().collect();

    for row in rows {
        let parts: Vec<&str> = row.split_whitespace().collect();

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

    (a, b)
}

pub fn parse_to_grid(_input: &str) -> Vec<Vec<char>> {
    let grid = _input
        .lines()
        .filter(|line| !line.trim().is_empty()) // Remove empty lines if present
        .map(|line| {
            line.chars()
                .map(|c| c) // Convert each char to a String
                .collect()
        })
        .collect();

    grid
}

#[derive(Hash, Debug, Clone, Eq, PartialEq, Default)]
pub struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug)]

pub struct Grid<T> {
    container: HashMap<Coord, T>,
    pub height: usize,
    pub width: usize,
}

impl Grid<char> {
    pub fn from_str(input: &str) -> Self {
        let mut container = HashMap::<Coord, char>::new();

        let height = input.lines().count();
        let width = input.len() / height;
        for (col, line) in input.lines().enumerate() {
            for (row, ch) in line.chars().enumerate() {
                container.insert(
                    Coord {
                        x: col as isize,
                        y: row as isize,
                    },
                    ch,
                );
            }
        }
        Self {
            container,
            height,
            width,
        }
    }
}

impl<T> Grid<T> {
    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&Coord { x, y })
    }
    pub fn north(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&Coord { x, y: y - 1 })
    }
    pub fn south(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&Coord { x, y: y + 1 })
    }
    pub fn east(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&Coord { x: x + 1, y })
    }
    pub fn west(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&Coord { x: x - 1, y })
    }
    pub fn northwest(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&Coord { x: x - 1, y: y - 1 })
    }
    pub fn northeast(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&Coord { x: x + 1, y: y - 1 })
    }
    pub fn southwest(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&Coord { x: x - 1, y: y + 1 })
    }
    pub fn southeast(&self, x: isize, y: isize) -> Option<&T> {
        self.container.get(&Coord { x: x + 1, y: y + 1 })
    }
}

pub const NEIGHBORS: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (0, -1),
    (1, -1),
    (-1, -1),
    (-1, 1),
    (-1, 0),
];

impl FromIterator<char> for Grid<char> {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let mut container = HashMap::<Coord, char>::new();
        let mut x: isize = 0;
        let mut y: isize = 0;
        let mut height = 0;
        let mut width = 0;
        for ch in iter {
            match ch {
                '\r' => (),
                '\n' => {
                    y += 1;
                    x = 0;
                }
                _ => {
                    container.insert(Coord { x, y }, ch);
                    height = height.max(y as usize + 1);
                    width = width.max(x as usize + 1);
                    x += 1;
                }
            }
        }

        Grid {
            container,
            height: height,
            width: width,
        }
    }
}
