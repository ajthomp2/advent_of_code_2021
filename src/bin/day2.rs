use std::error::Error;
use std::fs;

enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

struct Position(u32, i32);

#[derive(Debug)]
struct PositionAndAim {
    horizontal: u32,
    depth: i32,
    aim: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/data/day2_input.txt")?
        .trim()
        .split("\n")
        .map(|line| map_to_instruction(line))
        .collect::<Vec<Instruction>>();

    let result_part_1 = input.iter().fold(Position(0, 0), |acc, next| match next {
        Instruction::Forward(val) => Position(acc.0 + val, acc.1),
        Instruction::Down(val) => Position(acc.0, acc.1 + *val as i32),
        Instruction::Up(val) => Position(acc.0, acc.1 - *val as i32),
    });

    println!(
        "Part 1 Result: {}",
        result_part_1.0 as i32 * result_part_1.1
    );

    let result_part_2 = input.iter().fold(
        PositionAndAim {
            horizontal: 0,
            depth: 0,
            aim: 0,
        },
        |acc, next| match next {
            Instruction::Forward(val) => PositionAndAim {
                horizontal: acc.horizontal + val,
                depth: acc.depth + acc.aim * (*val as i32),
                aim: acc.aim,
            },
            Instruction::Down(val) => PositionAndAim {
                horizontal: acc.horizontal,
                depth: acc.depth,
                aim: acc.aim + *val as i32,
            },
            Instruction::Up(val) => PositionAndAim {
                horizontal: acc.horizontal,
                depth: acc.depth,
                aim: acc.aim - *val as i32,
            },
        },
    );

    println!(
        "Part 2 Result: {}",
        result_part_2.horizontal as i32 * result_part_2.depth // result_part_2
    );

    Ok(())
}

fn map_to_instruction(line: &str) -> Instruction {
    let (instruction, value) = line.split_once(" ").unwrap();
    let value = value.parse::<u32>().unwrap();
    match instruction {
        "forward" => Instruction::Forward(value),
        "down" => Instruction::Down(value),
        "up" => Instruction::Up(value),
        unexpected_instruction => {
            panic!("Unexpected Instruction: {}", unexpected_instruction)
        }
    }
}
