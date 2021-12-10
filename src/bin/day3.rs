use std::error::Error;
use std::fs;
use std::str;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/data/day3_input.txt")?
        .trim()
        .split("\n")
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let input_len = input.get(0).unwrap().len();
    let most_common: String = input
        .iter()
        .fold(Vec::<isize>::with_capacity(input_len), |mut acc, next| {
            for (i, c) in next.chars().enumerate() {
                if let Some(val) = acc.get_mut(i) {
                    *val += if c == '1' { 1 } else { -1 };
                } else {
                    acc.push(if c == '1' { 1 } else { -1 });
                }
            }
            acc
        })
        .iter()
        .map(|v| if *v > 0 { "1" } else { "0" })
        .collect::<Vec<&str>>()
        .join("");

    let gamma_rate = usize::from_str_radix(&most_common, 2)?;
    let epsilon_rate = binary_negate(gamma_rate, input_len);

    println!("Part 1 Result: {}", gamma_rate * epsilon_rate);

    let oxygen_gen_rating =
        usize::from_str_radix(most_common_iter(&mut input.clone()).unwrap(), 2)?;
    let co2_scrub_rating =
        usize::from_str_radix(least_common_iter(&mut input.clone()).unwrap(), 2)?;

    println!("Part 2 Result: {}", oxygen_gen_rating * co2_scrub_rating);

    Ok(())
}

fn binary_negate(val_to_negate: usize, len: usize) -> usize {
    let mask = usize::pow(2, len as u32) - 1;
    !val_to_negate & mask
}

fn most_common_iter(values: &mut Vec<String>) -> Option<&String> {
    remove_all_with_value_at_ind_iter(
        values,
        |most_common_at_ind| {
            if most_common_at_ind >= 0 {
                '1'
            } else {
                '0'
            }
        },
    )
}

fn least_common_iter(values: &mut Vec<String>) -> Option<&String> {
    remove_all_with_value_at_ind_iter(
        values,
        |most_common_at_ind| {
            if most_common_at_ind >= 0 {
                '0'
            } else {
                '1'
            }
        },
    )
}

fn remove_all_with_value_at_ind_iter<T>(
    values: &mut Vec<String>,
    value_determiner: T,
) -> Option<&String>
where
    T: Fn(isize) -> char,
{
    let mut index = 0;
    while values.len() > 1 {
        let most_common_at_ind = values.iter().fold(0, |mut acc, next| {
            acc += if next.as_bytes()[index] as char == '1' {
                1
            } else {
                -1
            };
            acc
        });
        let most_common_at_ind = value_determiner(most_common_at_ind);
        values.retain(|v| v.as_bytes()[index] as char == most_common_at_ind);
        index += 1;
    }
    values.get(0)
}
