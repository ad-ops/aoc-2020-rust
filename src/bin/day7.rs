#[macro_use] extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use aoc_2020_rust::{puzzle_main, puzzle_tests, Puzzle};
use regex::Regex;

fn parse(text: &str) -> (String, HashMap<String, usize>) {
    let mut bags: HashMap<String, usize> = HashMap::new();
    lazy_static! {
        static ref CONTAINER_RE: Regex = Regex::new(r"([ \w]+) bags contain").unwrap();
        static ref BAGS_RE: Regex = Regex::new(r"(\d+) ([ \w]+) bags?").unwrap();
    }
    let caps = CONTAINER_RE.captures(text).unwrap();
    let container = caps.get(1).unwrap().as_str().to_string();
    for cap in BAGS_RE.captures_iter(text) {
        let color = cap[2].to_string();
        let amount = cap[1].parse::<usize>().unwrap();
        bags.insert(color, amount);
    }
    
    (container, bags)
}

fn contains_gold(color: String, bags: &HashMap<String, HashMap<String, usize>>) -> bool {
    if let Some(bag) = bags.get(&color) {
        let golden_bag = "shiny gold";
        if bag.contains_key(golden_bag) {
            return true;
        }

        let mut found = false;
        for (container, _) in bag {
            found = found || contains_gold(container.to_string(), &bags);
        }
        return found;
    }
    false
}

fn bag_contains_amount(color: String, bags: &HashMap<String, HashMap<String, usize>>) -> usize {
    let mut count = 0;
    if let Some(bag) = bags.get(&color) {
        for (inner_bag, amount) in bag {
            count += amount;
            count += bag_contains_amount((*inner_bag).to_string(), bags) * amount;
        }
    }
    count
}

fn solver_part1(input: Vec<String>) -> String {
    let mut bags = HashMap::new();
    for line in input {
        let (container, content) = parse(&line);
        bags.insert(container, content);
    }

    let b = &bags;
    let solution = b
        .into_iter()
        .filter(|(color, _)| contains_gold((*color).to_string(), &b))
        .count();

    solution.to_string()
}

fn solver_part2(input: Vec<String>) -> String {
    let mut bags = HashMap::new();
    for line in input {
        let (container, content) = parse(&line);
        bags.insert(container, content);
    }
    let solution = bag_contains_amount("shiny gold".to_string(), &bags);
    solution.to_string()
}

puzzle_main!(solver_part1, solver_part2);

puzzle_tests!("4", "32");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn contains_gold_at_root() {
        let mut content: HashMap<String, usize> = HashMap::new();
        content.insert("shiny gold".to_string(), 2);

        let mut bags: HashMap<String, HashMap<String, usize>> = HashMap::new();
        bags.insert("bright white".to_string(), content);

        assert!(contains_gold("bright white".to_string(), &bags));
    }

    #[test]
    fn contains_gold_color_missing() {
        let bags: HashMap<String, HashMap<String, usize>> = HashMap::new();

        assert!(!contains_gold("bright white".to_string(), &bags));
    }

    #[test]
    fn contains_gold_one_deep() {
        let mut white_content: HashMap<String, usize> = HashMap::new();
        white_content.insert("dark blue".to_string(), 2);

        let mut blue_content: HashMap<String, usize> = HashMap::new();
        blue_content.insert("shiny gold".to_string(), 1);

        let mut bags: HashMap<String, HashMap<String, usize>> = HashMap::new();
        bags.insert("bright white".to_string(), white_content);
        bags.insert("dark blue".to_string(), blue_content);

        assert!(contains_gold("bright white".to_string(), &bags));
    }

    #[test]
    fn contains_gold_two_deep() {
        let mut red_content: HashMap<String, usize> = HashMap::new();
        red_content.insert("bright white".to_string(), 2);

        let mut white_content: HashMap<String, usize> = HashMap::new();
        white_content.insert("dark blue".to_string(), 2);

        let mut blue_content: HashMap<String, usize> = HashMap::new();
        blue_content.insert("shiny gold".to_string(), 1);

        let mut bags: HashMap<String, HashMap<String, usize>> = HashMap::new();
        bags.insert("bright white".to_string(), white_content);
        bags.insert("dark blue".to_string(), blue_content);
        bags.insert("smooth red".to_string(), red_content);

        assert!(contains_gold("smooth red".to_string(), &bags));
    }

    #[test]
    fn contains_gold_two_deep_does_not_contain() {
        let mut red_content: HashMap<String, usize> = HashMap::new();
        red_content.insert("bright white".to_string(), 2);

        let mut white_content: HashMap<String, usize> = HashMap::new();
        white_content.insert("dark blue".to_string(), 2);

        let mut blue_content: HashMap<String, usize> = HashMap::new();
        blue_content.insert("shiny silver".to_string(), 1);

        let mut bags: HashMap<String, HashMap<String, usize>> = HashMap::new();
        bags.insert("bright white".to_string(), white_content);
        bags.insert("dark blue".to_string(), blue_content);
        bags.insert("smooth red".to_string(), red_content);

        assert!(!contains_gold("smooth red".to_string(), &bags));
    }

    #[test]
    fn bag_contains_amount_two_deep() {
        let mut red_content: HashMap<String, usize> = HashMap::new();
        red_content.insert("bright white".to_string(), 2);

        let mut white_content: HashMap<String, usize> = HashMap::new();
        white_content.insert("dark blue".to_string(), 2);

        let mut blue_content: HashMap<String, usize> = HashMap::new();
        blue_content.insert("shiny silver".to_string(), 1);

        let mut bags: HashMap<String, HashMap<String, usize>> = HashMap::new();
        bags.insert("bright white".to_string(), white_content);
        bags.insert("dark blue".to_string(), blue_content);
        bags.insert("smooth red".to_string(), red_content);
        bags.insert("shiny silver".to_string(), HashMap::new());

        assert_eq!(10, bag_contains_amount("smooth red".to_string(), &bags));
    }

    #[test]
    fn bag_contains_amount_multiple() {
        let mut red_content: HashMap<String, usize> = HashMap::new();
        red_content.insert("bright white".to_string(), 2);
        red_content.insert("shiny silver".to_string(), 2);

        let mut white_content: HashMap<String, usize> = HashMap::new();
        white_content.insert("dark blue".to_string(), 2);

        let mut bags: HashMap<String, HashMap<String, usize>> = HashMap::new();
        bags.insert("smooth red".to_string(), red_content);
        bags.insert("bright white".to_string(), white_content);

        assert_eq!(8, bag_contains_amount("smooth red".to_string(), &bags));
    }
}