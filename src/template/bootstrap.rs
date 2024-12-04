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
