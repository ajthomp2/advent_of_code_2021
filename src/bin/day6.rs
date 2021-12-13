use std::error::Error;
use std::fs;

const NEW_TIMER: usize = 8;
const RESET_TIMER: usize = 6;
const PART_1_SIMULATION_DAYS: usize = 80;
const PART_2_SIMULATION_DAYS: usize = 256;

fn main() -> Result<(), Box<dyn Error>> {
    let mut timer_counts = fs::read_to_string("src/data/day6_input.txt")?
        .trim()
        .split(",")
        .map(|l| l.parse::<usize>().unwrap())
        .fold([0; NEW_TIMER + 1], |mut acc, input| {
            acc[input] += 1;
            acc
        });

    for day in 0..PART_2_SIMULATION_DAYS {
        let to_reset = timer_counts[0];
        for i in 1..NEW_TIMER + 1 {
            timer_counts[i - 1] = timer_counts[i];
        }
        timer_counts[RESET_TIMER] += to_reset;
        timer_counts[NEW_TIMER] = to_reset;

        if day == PART_1_SIMULATION_DAYS - 1 {
            println!("Part 1 Result: {}", timer_counts.iter().sum::<usize>());
        }
    }

    println!("Part 2 Result: {}", timer_counts.iter().sum::<usize>());

    Ok(())
}
