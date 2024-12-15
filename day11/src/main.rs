use std::collections::HashMap;

fn main() {
    let input = read_input();

    let mut cache: HashMap<(u64, u8), u64> = HashMap::new();

    let result_part_1: u64 = input
        .iter()
        .map(|&stone| blink_v2(stone, 25, &mut cache))
        .sum();

    println!("{}", result_part_1);

    let result_part_2: u64 = input
        .iter()
        .map(|&stone| blink_v2(stone, 75, &mut cache))
        .sum();

    println!("{}", result_part_2);
}

fn blink_v2(stone: u64, times_to_blink: u8, cache: &mut HashMap<(u64, u8), u64>) -> u64 {
    if let Some(&v) = cache.get(&(stone, times_to_blink)) {
        return v;
    } else if times_to_blink == 0 {
        return 1;
    }

    let value: u64;

    let as_string = stone.to_string();
    if stone == 0 {
        value = blink_v2(1, times_to_blink - 1, cache);
    } else if as_string.len() % 2 == 0 {
        value = blink_v2(
            as_string[0..as_string.len() / 2].parse().unwrap(),
            times_to_blink - 1,
            cache,
        ) + blink_v2(
            as_string[as_string.len() / 2..].parse().unwrap(),
            times_to_blink - 1,
            cache,
        );
    } else {
        value = blink_v2(stone * 2024, times_to_blink - 1, cache);
    }

    cache.insert((stone, times_to_blink), value);

    value
}

fn read_input() -> Vec<u64> {
    include_str!("../resources/input.txt")
        .trim_end()
        .split(" ")
        .map(|n| n.parse().expect("parse number"))
        .collect()
}
