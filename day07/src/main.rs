use std::cmp::max;

fn main() {
    let input = read_input();

    let result_part_1: u64 = input
        .iter()
        .filter(|&(target, operands)| can_resolve(0, 0, *target, operands, false))
        .map(|(value, _)| value)
        .sum();

    println!("{}", result_part_1);

    let result_part_2: u64 = input
        .iter()
        .filter(|&(target, operands)| can_resolve(0, 0, *target, operands, true))
        .map(|(value, _)| value)
        .sum();

    println!("{}", result_part_2);
}

fn can_resolve(
    current_value: u64,
    current_index: usize,
    target: u64,
    operands: &Vec<u64>,
    allow_concatenation: bool,
) -> bool {
    if current_index == operands.len() {
        current_value == target
    } else {
        can_resolve(
            current_value + operands[current_index],
            current_index + 1,
            target,
            operands,
            allow_concatenation,
        ) || can_resolve(
            max(current_value, 1) * operands[current_index],
            current_index + 1,
            target,
            operands,
            allow_concatenation,
        ) || (allow_concatenation
            && can_resolve(
                concatenate_numbers(current_value, operands[current_index]),
                current_index + 1,
                target,
                operands,
                allow_concatenation,
            ))
    }
}

fn concatenate_numbers(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b)
        .parse::<u64>()
        .expect("concatenate numbers")
}

fn read_input() -> Vec<(u64, Vec<u64>)> {
    include_str!("../resources/input.txt")
        .trim_end()
        .split("\n")
        .map(|line| map_line(line))
        .collect()
}

fn map_line(line: &str) -> (u64, Vec<u64>) {
    let split: Vec<&str> = line.split(": ").collect();
    let parts: Vec<u64> = split[1]
        .split(" ")
        .map(|num| num.parse().expect("parse number"))
        .collect();

    (split[0].parse().expect("parse number"), parts)
}
