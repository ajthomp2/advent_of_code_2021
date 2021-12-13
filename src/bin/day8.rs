use std::collections::HashMap;
use std::error::Error;
use std::fs;

const NUM_INPUTS: usize = 10;
const NUM_OUTPUTS: usize = 4;

struct Entry {
    inputs: [Vec<char>; NUM_INPUTS],
    outputs: [Vec<char>; NUM_OUTPUTS],
}

impl Entry {
    fn from_input(input: &str) -> Self {
        let (inputs, outputs) = input.split_once(" | ").unwrap();
        Entry {
            inputs: inputs
                .trim()
                .split_whitespace()
                .map(|i| {
                    let mut chars = i.chars().collect::<Vec<char>>();
                    chars.sort();
                    chars
                })
                .collect::<Vec<Vec<char>>>()
                .try_into()
                .unwrap(),
            outputs: outputs
                .trim()
                .split_whitespace()
                .map(|i| {
                    let mut chars = i.chars().collect::<Vec<char>>();
                    chars.sort();
                    chars
                })
                .collect::<Vec<Vec<char>>>()
                .try_into()
                .unwrap(),
        }
    }

    // 2 -> 1
    // 3 -> 7
    // 4 -> 4
    // 5 -> 2, 3, 5
    // 6 -> 6, 0, 9
    // 7 -> 8
    fn determine_output_value(self) -> usize {
        let len_to_input = self.inputs.into_iter().fold(
            HashMap::<usize, Vec<Vec<char>>>::with_capacity(6),
            |mut acc, input| {
                if let Some(inputs) = acc.get_mut(&input.len()) {
                    inputs.push(input);
                } else {
                    acc.insert(input.len(), vec![input]);
                }
                acc
            },
        );

        let mut input_to_num = HashMap::<&Vec<char>, usize>::with_capacity(10);
        // populate known values
        input_to_num.insert(&(*len_to_input.get(&2).unwrap())[0], 1);
        input_to_num.insert(&(*len_to_input.get(&4).unwrap())[0], 4);
        input_to_num.insert(&(*len_to_input.get(&3).unwrap())[0], 7);
        input_to_num.insert(&(*len_to_input.get(&7).unwrap())[0], 8);

        let two_three_and_five: &Vec<Vec<char>> = len_to_input.get(&5).unwrap();
        let zero_six_and_nine: &Vec<Vec<char>> = len_to_input.get(&6).unwrap();
        let one: &Vec<char> = &(*len_to_input.get(&2).unwrap())[0];

        // 6 doesn't contain both segments of 1
        let mut upper_right_segment = &' ';
        let mut six = &vec![];
        for input in zero_six_and_nine {
            if !input.contains(&one[0]) {
                upper_right_segment = &one[0];
                six = &input;
                input_to_num.insert(&input, 6);
            } else if !input.contains(&one[1]) {
                upper_right_segment = &one[1];
                six = &input;
                input_to_num.insert(&input, 6);
            }
        }

        // use five to figure out bottom left segment below
        let mut five = &vec![];
        for input in two_three_and_five {
            if input.contains(&one[0]) && input.contains(&one[1]) {
                input_to_num.insert(&input, 3);
            } else if input.contains(upper_right_segment) {
                input_to_num.insert(&input, 2);
            } else {
                five = &input;
                input_to_num.insert(&input, 5);
            }
        }

        let eight: &Vec<char> = &(*len_to_input.get(&7).unwrap())[0];
        let bottom_left_segment: &char = eight
            .iter()
            .filter(|c| !one.contains(c) && !five.contains(c))
            .collect::<Vec<&char>>()[0];
        for input in zero_six_and_nine {
            if input == six {
                continue;
            } else if !input.contains(bottom_left_segment) {
                input_to_num.insert(&input, 9);
            } else {
                input_to_num.insert(&input, 0);
            }
        }

        let mut result = 0;
        for i in 0..self.outputs.len() {
            let multiplier: usize = 1000 / usize::pow(10, i as u32);
            result += *input_to_num.get(&self.outputs[i]).unwrap() * multiplier;
        }
        result
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let entries = fs::read_to_string("src/data/day8_input.txt")?
        .trim()
        .split("\n")
        .map(|l| Entry::from_input(l))
        .collect::<Vec<Entry>>();

    let count_of_1_4_7_8 = entries
        .iter()
        .map(|e| {
            e.outputs
                .iter()
                .filter(|o| o.len() == 2 || o.len() == 3 || o.len() == 4 || o.len() == 7)
                .count()
        })
        .sum::<usize>();

    println!("Part 1 Result: {}", count_of_1_4_7_8);

    let output_sum = entries
        .into_iter()
        .map(|e| e.determine_output_value())
        .sum::<usize>();

    println!("Part 2 Result: {}", output_sum);

    Ok(())
}
