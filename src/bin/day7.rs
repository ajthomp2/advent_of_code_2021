use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut inputs = fs::read_to_string("src/data/day7_input.txt")?
        .trim()
        .split(",")
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();

    inputs.sort();

    let median = if inputs.len() % 2 == 1 {
        (*inputs.get(inputs.len() / 2).unwrap() + *inputs.get(inputs.len() / 2 - 1).unwrap()) / 2
    } else {
        *inputs.get(inputs.len() / 2).unwrap()
    };

    println!(
        "Part 1 Result: {}",
        inputs.iter().map(|i| (i - median).abs()).sum::<isize>()
    );

    // takes median as starting place
    // tests step in one direction. if error decreases, then keep going, otherwise go other direction
    // step in that direction until error is min
    let median_error = part_2_error_calc(&inputs, median);
    let canary_error = part_2_error_calc(&inputs, median - 1);
    let (step, mut start, mut curr_error) = if median_error < canary_error {
        (1, median, median_error)
    } else {
        (-1, median - 1, canary_error)
    };

    let mut prev_error = isize::MAX;
    while curr_error < prev_error {
        prev_error = curr_error;
        curr_error = part_2_error_calc(&inputs, start + step);
        start += step;
    }

    println!("Part 2 Result: {}", prev_error);

    Ok(())
}

fn part_2_error_calc(inputs: &Vec<isize>, val: isize) -> isize {
    inputs
        .iter()
        .map(|i| {
            let diff = (i - val).abs();
            (diff * (diff + 1)) / 2
        })
        .sum::<isize>()
}
