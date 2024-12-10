static EMPTY: i32 = -1;

fn main() {
    let input = read_input();

    part_1(input.clone());

    part_2(input.clone());
}

fn part_1(mut input: Vec<i32>) {
    while input.contains(&EMPTY) {
        // pop last element
        let popped = input.pop().expect("last element");

        if popped != EMPTY {
            let first_empty_index = input
                .iter()
                .position(|&x| x == EMPTY)
                .expect("empty position");
            input[first_empty_index] = popped;
        }
    }

    println!("{}", calculate_checksum(&input));
}

fn part_2(mut input: Vec<i32>) {
    let max_file_index = input.iter().max().expect("max file index");

    for file_index in (0..=*max_file_index).rev() {
        // find file size
        let size = input.iter().filter(|&&x| x == file_index).count();
        let file_start = input
            .iter()
            .position(|&x| x == file_index)
            .expect("position");

        // find a gap
        if let Some(gap_start) = find_gap_of_size(&input, size, file_start) {
            // remove file
            for x in 0..size {
                input[file_start + x] = EMPTY;
            }

            // copy file to gap
            for i in 0..size {
                input[gap_start + i] = file_index;
            }
        }
    }

    println!("{}", calculate_checksum(&input));
}

fn find_gap_of_size(filesystem: &Vec<i32>, size: usize, before: usize) -> Option<usize> {
    for i in 0..before {
        if filesystem[i..i + size].iter().all(|&x| x == EMPTY) {
            return Some(i);
        }
    }

    None
}

fn calculate_checksum(filesystem: &Vec<i32>) -> u64 {
    filesystem
        .iter()
        .enumerate()
        .filter(|&(_, &x)| x != EMPTY)
        .map(|(position, file_index)| position as u64 * *file_index as u64)
        .sum()
}

fn read_input() -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    let mut is_file = true;
    let mut file_index: u32 = 0;

    include_str!("../resources/input.txt")
        .trim()
        .chars()
        .for_each(|c| {
            let n: usize = c as usize - '0' as usize;

            if is_file {
                result.resize(result.len() + n, file_index as i32);

                file_index += 1;
            } else {
                result.resize(result.len() + n, EMPTY);
            }

            is_file = !is_file;
        });

    result
}
