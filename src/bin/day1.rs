use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let depths = fs::read_to_string("src/data/day1_input.txt")?
        .trim()
        .split("\n")
        .map(|line| line.to_string().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut prev: &u32 = depths.get(0).unwrap();
    let mut next_value_is_greater = 0;
    for depth in depths.iter().skip(1) {
        if depth > prev {
            next_value_is_greater += 1
        }
        prev = &depth;
    }

    println!("Part 1 Result: {}", next_value_is_greater);

    let depth_window_averages = depths
        .windows(3)
        .map(|window| window[0] + window[1] + window[2])
        .collect::<Vec<u32>>();

    let mut prev: &u32 = depth_window_averages.get(0).unwrap();
    let mut next_value_is_greater = 0;
    for depth_window_average in depth_window_averages.iter().skip(1) {
        if depth_window_average > prev {
            next_value_is_greater += 1
        }
        prev = &depth_window_average;
    }

    println!("Part 2 Result: {}", next_value_is_greater);

    Ok(())
}
