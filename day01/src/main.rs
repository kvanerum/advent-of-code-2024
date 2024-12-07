use std::iter::zip;

fn main() {
    let input: (Vec<u32>, Vec<u32>) = include_str!("../resources/input.txt")
        .trim_end()
        .split("\n")
        .map(|line| {
            line.split("   ")
                .map(|s| s.parse::<u32>().expect("parse number"))
                .collect()
        })
        .map(|parts: Vec<u32>| (parts[0], parts[1]))
        .unzip();

    let mut left = input.0.clone();
    left.sort();
    let mut right = input.1.clone();
    right.sort();

    let mut distance: u32 = 0;

    for i in zip(left, right) {
        distance += i.0.abs_diff(i.1);
    }

    println!("{distance}");

    let mut similarity: u32 = 0;

    for left_item in input.0 {
        let num_occurrences = input
            .1
            .iter()
            .filter(|right_item| left_item == **right_item)
            .count() as u32;

        similarity += left_item * num_occurrences;
    }

    println!("{similarity}");
}
