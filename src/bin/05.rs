advent_of_code::solution!(5);

use std::collections::{HashMap, HashSet, VecDeque};

type Graph = HashMap<u32, HashSet<u32>>;

fn parse_rules(input: &str) -> Graph {
    let mut graph: Graph = HashMap::new();
    for line in input.lines() {
        if let Some((a, b)) = line.split_once('|') {
            let a: u32 = a.trim().parse().unwrap();
            let b: u32 = b.trim().parse().unwrap();
            graph.entry(a).or_insert_with(HashSet::new).insert(b);
        }
    }
    graph
}

fn parse_updates(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split(',')
                .map(|num| num.trim().parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_valid_update(update: &[u32], rules: &Graph) -> bool {
    let mut index_map = HashMap::new();
    for (i, &page) in update.iter().enumerate() {
        index_map.insert(page, i);
    }
    for (&a, dependencies) in rules {
        for &b in dependencies {
            if let (Some(&a_index), Some(&b_index)) = (index_map.get(&a), index_map.get(&b)) {
                if a_index > b_index {
                    return false;
                }
            }
        }
    }
    true
}

fn find_middle_page(update: &[u32]) -> u32 {
    update[update.len() / 2]
}

fn topological_sort(update: &[u32], rules: &Graph) -> Vec<u32> {
    let mut graph = HashMap::new();
    let mut indegree = HashMap::new();
    let pages: HashSet<u32> = update.iter().cloned().collect();

    for &page in &pages {
        graph.entry(page).or_insert_with(Vec::new);
        indegree.entry(page).or_insert(0);
    }

    for (&a, dependencies) in rules {
        if !pages.contains(&a) {
            continue;
        }
        for &b in dependencies {
            if pages.contains(&b) {
                graph.get_mut(&a).unwrap().push(b);
                *indegree.entry(b).or_insert(0) += 1;
            }
        }
    }
    let mut queue = VecDeque::new();
    for (&page, &degree) in &indegree {
        if degree == 0 {
            queue.push_back(page);
        }
    }

    let mut sorted = Vec::new();
    while let Some(page) = queue.pop_front() {
        sorted.push(page);
        for &neighbor in &graph[&page] {
            if let Some(indegree_count) = indegree.get_mut(&neighbor) {
                *indegree_count -= 1;
                if *indegree_count == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    sorted
}

pub fn part_one(_input: &str) -> Option<u32> {
    let sections: Vec<&str> = _input
        .split("\n\n")
        .filter(|s| !s.trim().is_empty())
        .collect();

    let rules = parse_rules(sections[0]);
    let updates = parse_updates(sections[1]);

    let mut valid_middle_pages = vec![];
    for update in &updates {
        if is_valid_update(update, &rules) {
            valid_middle_pages.push(find_middle_page(update));
        }
    }

    let sum = valid_middle_pages.iter().sum();
    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let sections: Vec<&str> = _input
        .split("\n\n")
        .filter(|s| !s.trim().is_empty())
        .collect();

    let rules = parse_rules(sections[0]);
    let updates = parse_updates(sections[1]);

    let mut corrected_middle_pages = vec![];
    for update in &updates {
        if !is_valid_update(update, &rules) {
            let corrected_update = topological_sort(update, &rules);
            corrected_middle_pages.push(find_middle_page(&corrected_update));
        }
    }

    let sum = corrected_middle_pages.iter().sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
