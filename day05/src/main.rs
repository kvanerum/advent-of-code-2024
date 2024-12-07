use std::collections::HashSet;

fn main() {
    let input = read_input();

    // for (index, ordering) in input.1.iter().enumerate() {
    //     let valid = is_valid(ordering, &input.0);
    //     println!("item {index}: {valid}");
    // }

    let result_part_1: u32 = input
        .1
        .iter()
        .filter(|ordering| is_valid(ordering, &input.0))
        .map(|ordering| ordering[ordering.len() / 2] as u32)
        .sum();

    println!("{result_part_1}");

    let result_part_2: u32 = input
        .1
        .iter()
        .filter(|ordering| !is_valid(ordering, &input.0))
        .map(|ordering| fix_order(ordering, &input.0))
        .map(|ordering| ordering[ordering.len() / 2] as u32)
        .sum();

    println!("{result_part_2}");
}

fn fix_order(input: &Vec<u8>, rules: &Vec<(u8, u8)>) -> Vec<u8> {
    let mut pages: HashSet<u8> = HashSet::from_iter(input.iter().cloned());
    let mut result: Vec<u8> = Vec::with_capacity(input.len());

    while !pages.is_empty() {
        // find element that always is in front of other elements
        let next_page = *pages
            .iter()
            .find(|&&p| {
                rules.iter().all(|rule| {
                    (rule.0 != p && rule.1 != p)
                        || (rule.0 == p)
                        || (rule.1 == p && !pages.contains(&rule.0))
                })
            })
            .expect("find next page");
        
        pages.remove(&next_page);
        result.push(next_page);
    }

    result
}

fn is_valid(order: &Vec<u8>, rules: &Vec<(u8, u8)>) -> bool {
    for i in 0..order.len() {
        let first = order[i];
        for j in i + 1..order.len() {
            let second = order[j];

            if rules.iter().any(|rule| rule.1 == first && rule.0 == second) {
                return false;
            }
        }
    }

    true
}

fn read_input() -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
    let input = include_str!("../resources/input.txt").trim_end();
    let parts: Vec<&str> = input.split("\n\n").collect();

    let rules: Vec<(u8, u8)> = parts[0]
        .split("\n")
        .map(|line| {
            let numbers: Vec<u8> = line
                .split("|")
                .map(|n| n.parse::<u8>().expect("parse number"))
                .collect();
            (numbers[0], numbers[1])
        })
        .collect();

    let orderings: Vec<Vec<u8>> = parts[1]
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<u8>().expect("parse number"))
                .collect()
        })
        .collect();

    (rules, orderings)
}
