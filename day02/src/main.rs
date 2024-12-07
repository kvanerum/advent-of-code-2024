fn main() {
    let input: Vec<Vec<i8>> = include_str!("../resources/input.txt")
        .trim_end()
        .split("\n")
        .map(|line| {
            line.split(" ")
                .map(|n| n.parse::<i8>().expect("parse number"))
                .collect()
        })
        .collect();

    let num_safe_part_1 = input.iter().filter(|report| is_safe(report)).count();

    println!("{num_safe_part_1}");

    let num_safe_part_2 = input
        .iter()
        .filter(|report| is_safe(report) || is_safe_with_dampener(report))
        .count();

    println!("{num_safe_part_2}");
}

fn is_safe_with_dampener(report: &&Vec<i8>) -> bool {
    for i in 0..report.len() {
        let dampened: Vec<i8> = report
            .iter()
            .enumerate()
            .filter_map(|(idx, &val)| if idx != i { Some(val) } else { None })
            .collect();

        if is_safe(&&dampened) {
            return true;
        }
    }

    false
}

fn is_safe(report: &&Vec<i8>) -> bool {
    if report.len() < 2 {
        return true;
    }

    let ascending = report[1] > report[0];

    for i in 0..report.len() - 1 {
        if (ascending && (report[i + 1] <= report[i] || report[i + 1] > report[i] + 3))
            || (!ascending && (report[i + 1] >= report[i] || report[i + 1] < report[i] - 3))
        {
            return false;
        }
    }

    true
}
