use regex::Regex;

fn main() {
    let input: &str = include_str!("../resources/input.txt").trim_end();
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("regex");

    // part 1
    let mut result = process_line(input, &regex);

    println!("{result}");

    // part 2
    let enable_needle = "do()";
    let disable_needle = "don't()";

    result = 0;

    let mut position = 0;

    while position < input.len() {
        if let Some(p) = input[position..].find(disable_needle) {
            result += process_line(&input[position..position + p], &regex);

            // set position after next do() or end of string
            position = input[position + p..]
                .find(enable_needle)
                .map(|x| return x + position + p)
                .unwrap();
        } else {
            // no more disabler, process until end
            result += process_line(&input[position..], &regex);
            position = input.len();
        }
    }

    println!("{result}");
}

fn process_line(line: &str, regex: &Regex) -> u32 {
    regex
        .captures_iter(line)
        .map(|capture| {
            let left = capture
                .get(1)
                .expect("left argument")
                .as_str()
                .parse::<u32>()
                .expect("parse number");

            let right = capture
                .get(2)
                .expect("right argument")
                .as_str()
                .parse::<u32>()
                .expect("parse number");

            return left * right;
        })
        .sum::<u32>()
}
