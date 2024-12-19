use std::collections::HashMap;

fn main() {
    let (register, instructions) = read_input();

    let output = run_program(register.clone(), &instructions);

    println!(
        "{}",
        output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );
}

fn run_program_with_register_a(
    register: &HashMap<char, usize>,
    instructions: &Vec<u8>,
    register_a: usize,
) -> Vec<u8> {
    let mut altered_register = register.clone();
    altered_register.insert('A', register_a);

    run_program(altered_register, instructions)
}

fn run_program(mut register: HashMap<char, usize>, instructions: &Vec<u8>) -> Vec<u8> {
    let mut instruction_pointer: usize = 0;
    let mut output: Vec<u8> = Vec::new();

    while instruction_pointer < instructions.len() {
        let opcode = instructions[instruction_pointer];
        let operand = instructions[instruction_pointer + 1];
        let mut jumped = false;

        match opcode {
            0 => {
                let denominator = 2usize.pow(get_combo_operand(operand, &register) as u32);
                let nominator = get_register_value(&register, 'A');
                register.insert('A', nominator / denominator);
            }
            1 => {
                let register_b = get_register_value(&register, 'B');
                register.insert('B', register_b ^ operand as usize);
            }
            2 => {
                let combo_operand = get_combo_operand(operand, &register);
                register.insert('B', combo_operand % 8);
            }
            3 => {
                let register_a = get_register_value(&register, 'A');
                if register_a != 0 {
                    instruction_pointer = operand as usize;
                    jumped = true;
                }
            }
            4 => {
                let register_b = get_register_value(&register, 'B');
                let register_c = get_register_value(&register, 'C');
                register.insert('B', register_b ^ register_c);
            }
            5 => {
                let combo_operand = get_combo_operand(operand, &register);
                output.push((combo_operand % 8) as u8);
            }
            6 => {
                let denominator = 2usize.pow(get_combo_operand(operand, &register) as u32);
                let nominator = get_register_value(&register, 'A');
                register.insert('B', nominator / denominator);
            }
            7 => {
                let denominator = 2usize.pow(get_combo_operand(operand, &register) as u32);
                let nominator = get_register_value(&register, 'A');
                register.insert('C', nominator / denominator);
            }
            _ => todo!("{}", opcode),
        }

        if !jumped {
            instruction_pointer += 2;
        }
    }

    output
}

fn get_register_value(register: &HashMap<char, usize>, name: char) -> usize {
    *register.get(&name).expect("register value")
}

fn get_combo_operand(operand: u8, register: &HashMap<char, usize>) -> usize {
    match operand {
        0..=3 => operand as usize,
        4 => register.get(&'A').expect("register value").clone(),
        5 => register.get(&'B').expect("register value").clone(),
        6 => register.get(&'C').expect("register value").clone(),
        _ => panic!("Invalid combo operand: {}", operand),
    }
}

fn read_input() -> (HashMap<char, usize>, Vec<u8>) {
    let parts: Vec<&str> = include_str!("../resources/input.txt")
        .trim_end()
        .split("\n\n")
        .collect();

    let register = parts[0]
        .split('\n')
        .map(|line| {
            let register = line.chars().nth(9).expect("register name");
            let value = line[12..].parse().expect("parse number");

            (register, value)
        })
        .collect();

    let instructions = parts[1][9..]
        .split(",")
        .map(|line| line.parse().expect("parse number"))
        .collect();

    (register, instructions)
}
