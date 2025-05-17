use std::collections::{HashMap, HashSet};

fn main() {
    let input = read_input();

    let numeric_keypad = HashMap::from([
        ('A', (3, 2)),
        ('0', (3, 1)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
    ]);
    let mut numeric_keypad_sequence = create_sequence_map(&numeric_keypad);

    numeric_keypad_sequence.insert(('7', '5'), String::from("v>"));
    numeric_keypad_sequence.insert(('4', '2'), String::from("v>"));
    numeric_keypad_sequence.insert(('5', 'A'), String::from("vv>"));
    numeric_keypad_sequence.insert(('8', '6'), String::from("v>"));
    numeric_keypad_sequence.insert(('7', '3'), String::from("vv>>"));
    numeric_keypad_sequence.insert(('4', '2'), String::from("v>"));
    numeric_keypad_sequence.insert(('4', '3'), String::from("v>>"));
    numeric_keypad_sequence.insert(('7', '5'), String::from("v>"));
    numeric_keypad_sequence.insert(('5', '3'), String::from("v>"));
    numeric_keypad_sequence.insert(('7', '2'), String::from("vv>"));
    numeric_keypad_sequence.insert(('8', 'A'), String::from("vvv>"));
    numeric_keypad_sequence.insert(('8', '3'), String::from("vv>"));
    numeric_keypad_sequence.insert(('2', 'A'), String::from("v>"));
    numeric_keypad_sequence.insert(('7', '6'), String::from("v>>"));

    let directional_keypad = HashMap::from([
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ]);

    let mut directional_keypad_sequence = create_sequence_map(&directional_keypad);

    directional_keypad_sequence.insert(('^', '>'), String::from("v>"));
    directional_keypad_sequence.insert(('v', 'A'), String::from("^>"));

    let mut cache = HashMap::new();

    let part_1 = input
        .iter()
        .map(|code| {
            calculate_complexity(
                code.to_string(),
                &numeric_keypad_sequence,
                &directional_keypad_sequence,
                2,
                &mut cache,
            )
        })
        .sum::<usize>();

    println!("{part_1}");

    let part_2 = input
        .iter()
        .map(|code| {
            calculate_complexity(
                code.to_string(),
                &numeric_keypad_sequence,
                &directional_keypad_sequence,
                25,
                &mut cache,
            )
        })
        .sum::<usize>();

    println!("{part_2}");
}

fn calculate_complexity(
    input: String,
    numeric_keypad_sequence: &HashMap<(char, char), String>,
    directional_keypad_sequence: &HashMap<(char, char), String>,
    number_of_robots: usize,
    cache: &mut HashMap<(String, usize), usize>,
) -> usize {
    let result: usize = type_on_keypad(&input, numeric_keypad_sequence)
        .into_iter()
        .map(|part| resolve(part, number_of_robots, &directional_keypad_sequence, cache))
        .sum();

    result * input.replace("A", "").parse::<usize>().unwrap()
}

fn resolve(
    sequence: String,
    robots: usize,
    directional_keypad_sequence: &HashMap<(char, char), String>,
    cache: &mut HashMap<(String, usize), usize>,
) -> usize {
    if cache.contains_key(&(sequence.clone(), robots)) {
        return cache.get(&(sequence, robots)).unwrap().clone();
    }

    if robots == 0 {
        return sequence.len();
    }

    let mut result = 0;

    for part in type_on_keypad(&sequence, directional_keypad_sequence).into_iter() {
        result += &resolve(part, robots - 1, directional_keypad_sequence, cache);
    }

    cache.insert((sequence, robots), result.clone());

    result
}

fn type_on_keypad(input: &str, keypad_sequences: &HashMap<(char, char), String>) -> Vec<String> {
    let mut current_position = 'A';
    let mut result = Vec::new();

    for c in input.chars() {
        let next = keypad_sequences.get(&(current_position, c)).unwrap();
        let mut s = next.clone();
        s.push('A');
        result.push(s);
        current_position = c;
    }

    result
}

fn create_sequence_map(keypad: &HashMap<char, (usize, usize)>) -> HashMap<(char, char), String> {
    let mut result = HashMap::new();
    let valid_positions = keypad.values().collect::<HashSet<_>>();

    for from in keypad.keys() {
        for to in keypad.keys() {
            let sequence = move_to_position(
                *keypad.get(from).unwrap(),
                *keypad.get(to).unwrap(),
                &valid_positions,
            );

            result.insert((*from, *to), sequence);
        }
    }

    result
}

fn move_to_position(
    current_position: (usize, usize),
    target: (usize, usize),
    valid_positions: &HashSet<&(usize, usize)>,
) -> String {
    let mut sequence = String::new();

    // try horizontal
    let mut horizontal_move_done = false;
    if valid_positions.contains(&(current_position.0, target.1)) {
        let c = if current_position.1 < target.1 {
            '>'
        } else {
            '<'
        };
        for _ in 0..current_position.1.abs_diff(target.1) {
            sequence.push(c)
        }
        horizontal_move_done = true;
    }

    // vertical
    let c = if current_position.0 < target.0 {
        'v'
    } else {
        '^'
    };
    for _ in 0..current_position.0.abs_diff(target.0) {
        sequence.push(c)
    }

    if !horizontal_move_done {
        let c = if current_position.1 < target.1 {
            '>'
        } else {
            '<'
        };
        for _ in 0..current_position.1.abs_diff(target.1) {
            sequence.push(c)
        }
    }

    sequence
}

fn read_input() -> Vec<String> {
    include_str!("../resources/input.txt")
        .trim_end()
        .split("\n")
        .map(|s| s.to_string())
        .collect()
}
