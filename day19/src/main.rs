use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

fn main() {
    let (available_towels, designs) = read_input();

    let results: Vec<usize> = designs
        .par_iter()
        .map(|design| {
            count_possibilities(design, 0, &available_towels, &Mutex::new(HashMap::new()))
        })
        .collect();

    println!("{}", results.iter().filter(|&&count| count > 0).count());
    println!("{}", results.iter().sum::<usize>());
}

fn count_possibilities(
    design: &Vec<char>,
    current_position: usize,
    towels: &Vec<Vec<char>>,
    cache: &Mutex<HashMap<usize, usize>>,
) -> usize {
    {
        let cache_guard = cache.lock().unwrap();

        if current_position == design.len() {
            return 1;
        } else if cache_guard.contains_key(&current_position) {
            return *cache_guard.get(&current_position).unwrap();
        }
    }

    let result = towels
        .iter()
        .filter(|towel| towel_matches_next_stripes(design, current_position, towel))
        .map(|towel| count_possibilities(design, current_position + towel.len(), towels, cache))
        .sum();

    {
        let mut cache_guard = cache.lock().unwrap();
        cache_guard.insert(current_position, result);
    }

    result
}

fn towel_matches_next_stripes(
    design: &Vec<char>,
    current_position: usize,
    towel: &Vec<char>,
) -> bool {
    current_position + towel.len() <= design.len()
        && towel
            .iter()
            .enumerate()
            .all(|(offset, &stripe)| stripe == design[current_position + offset])
}

fn read_input() -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let mut parts = include_str!("../resources/input.txt")
        .trim_end()
        .split("\n\n");

    (
        parts
            .nth(0)
            .expect("part 1")
            .split(", ")
            .map(|x| x.chars().collect())
            .collect(),
        parts
            .nth(0)
            .expect("part 2")
            .split("\n")
            .map(|line| line.chars().collect())
            .collect(),
    )
}
