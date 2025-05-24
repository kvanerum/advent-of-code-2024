use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::collections::HashSet;
use std::ops::BitXor;

fn main() {
    let input = read_input();

    let sequences: Vec<Vec<u64>> = input
        .par_iter()
        .map(|number| {
            let mut sequence = Vec::new();
            sequence.push(*number);
            let mut next = *number;

            for _ in 0..2000 {
                next = calculate_next(next);
                sequence.push(next);
            }

            sequence
        })
        .collect();

    let part_1 = sequences.par_iter().map(|s| s.last().unwrap()).sum::<u64>();

    println!("{}", part_1);

    let prices: Vec<Vec<u8>> = sequences
        .iter()
        .map(|sequence| sequence.iter().map(|n| (n % 10) as u8).collect())
        .collect::<Vec<_>>();

    let deltas: Vec<Vec<i8>> = prices
        .iter()
        .map(|price_list| {
            price_list
                .windows(2)
                .map(|window| window[1] as i16 - window[0] as i16)
                .map(|delta| delta as i8)
                .collect()
        })
        .collect();

    let all_sequences = get_all_sequences(&deltas);
    
    let part_2 = all_sequences
        .par_iter()
        .progress_count(all_sequences.len() as u64)
        .map(|sequence| find_total_bananas(&deltas, &prices, sequence))
        .max()
        .unwrap();

    println!("{}", part_2);
}

fn get_all_sequences(deltas: &Vec<Vec<i8>>) -> HashSet<&[i8; 4]> {
    let mut result: HashSet<&[i8; 4]> = HashSet::new();

    deltas.iter().for_each(|delta| {
        delta.windows(4).for_each(|window| {
            result.insert(<&[i8; 4]>::try_from(window).unwrap());
        })
    });

    result
}

fn find_total_bananas(deltas: &Vec<Vec<i8>>, prices: &Vec<Vec<u8>>, sequence: &[i8; 4]) -> usize {
    (0..prices.len())
        .into_par_iter()
        .map(|i| find_price(&deltas[i], &prices[i], sequence))
        .filter(|price| price.is_some())
        .map(|price| price.unwrap())
        .map(|price| price as usize)
        .sum()
}

fn find_price(deltas: &Vec<i8>, prices: &Vec<u8>, sequence: &[i8; 4]) -> Option<u8> {
    for start in 0..deltas.len() - 3 {
        if deltas[start] == sequence[0]
            && deltas[start + 1] == sequence[1]
            && deltas[start + 2] == sequence[2]
            && deltas[start + 3] == sequence[3]
        {
            return Some(prices[start + 4]);
        }
    }

    None
}

fn calculate_next(number: u64) -> u64 {
    let mut result = (number * 64).bitxor(number) % 16777216;
    result = (result / 32).bitxor(result) % 16777216;
    result = (result * 2048).bitxor(result) % 16777216;

    result
}

fn read_input() -> Vec<u64> {
    include_str!("../resources/input.txt")
        .trim_end()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect()
}
