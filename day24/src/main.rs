use std::collections::HashMap;

#[derive(Debug)]
struct Instruction {
    input_1: &'static str,
    input_2: &'static str,
    op: &'static str,
    target: &'static str,
}
fn main() {
    let (state, instructions) = read_input();

    let part_1_state = run(&state, &instructions);

    let part_1: u64 = part_1_state
        .iter()
        .filter(|(key, _)| key.starts_with("z"))
        .filter(|(_, &value)| value)
        .map(|(key, _)| {
            key.chars()
                .skip(1)
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        })
        .map(|pow| 2_u64.pow(pow))
        .sum();
    println!("{}", part_1);

    let mut overflow_register: Option<&str> = None;

    for i in 0..45 {
        println!("{i}:");
        let x_str = format!("x{:0>2}", i);
        let y_str = format!("y{:0>2}", i);

        let xor_x_y = find_instruction(&instructions, &x_str, &y_str, "XOR").unwrap();
        println!("xor_x_y: {xor_x_y:?}");

        let and_x_y = find_instruction(&instructions, &x_str, &y_str, "AND").unwrap();
        println!("and_x_y: {and_x_y:?}");

        if let Some(o) = overflow_register {
            let xor_2 =
                find_instruction(&instructions, xor_x_y.target, o, "XOR").unwrap();
            println!("xor_2: {xor_2:?}");

            let and_2 =
                find_instruction(&instructions, xor_x_y.target, o, "AND").unwrap();
            println!("and_2: {and_2:?}");

            let or = find_instruction(&instructions, and_2.target, and_x_y.target, "OR").unwrap();
            println!("or: {or:?}");
            overflow_register = Some(or.target);
        } else {
            overflow_register = Some(and_x_y.target);
        }

        println!();
    }
}

fn find_instruction<'a>(
    instructions: &'a Vec<Instruction>,
    input_1: &str,
    input_2: &str,
    op: &str,
) -> Option<&'a Instruction> {
    instructions.iter().find(|instruction| {
        ((instruction.input_1 == input_1 && instruction.input_2 == input_2)
            || (instruction.input_1 == input_2 && instruction.input_2 == input_1))
            && instruction.op == op
    })
}

fn run(
    input_state: &HashMap<&'static str, bool>,
    instructions: &Vec<Instruction>,
) -> HashMap<&'static str, bool> {
    let mut state = input_state.clone();

    while let Some(instruction_to_solve) = instructions
        .iter()
        .filter(|instruction| {
            !state.contains_key(instruction.target)
                && state.contains_key(instruction.input_1)
                && state.contains_key(instruction.input_2)
        })
        .next()
    {
        state.insert(
            instruction_to_solve.target,
            match instruction_to_solve.op {
                "OR" => state[instruction_to_solve.input_1] || state[instruction_to_solve.input_2],
                "AND" => state[instruction_to_solve.input_1] && state[instruction_to_solve.input_2],
                "XOR" => state[instruction_to_solve.input_1] != state[instruction_to_solve.input_2],
                _ => panic!("Unknown op: {}", instruction_to_solve.op),
            },
        );
    }

    state
}

fn read_input() -> (HashMap<&'static str, bool>, Vec<Instruction>) {
    let parts: Vec<&str> = include_str!("../resources/input.txt")
        .trim_end()
        .split("\n\n")
        .collect();

    let state = parts
        .get(0)
        .unwrap()
        .split("\n")
        .map(|line| {
            let split: Vec<&str> = line.split(": ").collect();
            (split[0], split[1] == "1")
        })
        .collect();

    let instructions = parts
        .get(1)
        .unwrap()
        .split("\n")
        .map(|line| {
            let split: Vec<&str> = line.split(" ").collect();
            Instruction {
                input_1: split[0],
                input_2: split[2],
                op: split[1],
                target: split[4],
            }
        })
        .collect();

    (state, instructions)
}
