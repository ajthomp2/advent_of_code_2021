use std::convert::TryInto;
use std::error::Error;
use std::fs;

const BINGO_GRID_SIZE: usize = 5;

#[derive(Debug)]
struct Number {
    value: usize,
    selected: bool,
}

impl Number {
    fn new(value: &str) -> Number {
        Number {
            value: value.parse::<usize>().unwrap(),
            selected: false,
        }
    }
}

#[derive(Debug)]
struct Board {
    numbers: [[Number; BINGO_GRID_SIZE]; BINGO_GRID_SIZE],
    sum_unselected: usize,
    row_selected_counts: [usize; BINGO_GRID_SIZE],
    col_selected_counts: [usize; BINGO_GRID_SIZE],
    winner: bool,
}

impl Board {
    fn new(input: &str) -> Board {
        let mut sum_unselected = 0;
        Board {
            numbers: input
                .split("\n")
                .map(|row| {
                    row.split_whitespace()
                        .map(|l| {
                            let num = Number::new(l);
                            sum_unselected += num.value;
                            num
                        })
                        .collect::<Vec<Number>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<[Number; BINGO_GRID_SIZE]>>()
                .try_into()
                .unwrap(),
            sum_unselected: sum_unselected,
            row_selected_counts: [0; BINGO_GRID_SIZE],
            col_selected_counts: [0; BINGO_GRID_SIZE],
            winner: false,
        }
    }

    fn mark_number_and_set_if_winner(&mut self, draw: usize) {
        for i in 0..BINGO_GRID_SIZE {
            for j in 0..BINGO_GRID_SIZE {
                if self.numbers[i][j].value == draw {
                    self.numbers[i][j].selected = true;
                    self.sum_unselected -= draw;
                    self.row_selected_counts[i] += 1;
                    self.col_selected_counts[j] += 1;
                    self.winner = self.row_selected_counts[i] == BINGO_GRID_SIZE
                        || self.col_selected_counts[j] == BINGO_GRID_SIZE;
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/data/day4_input.txt")?
        .trim()
        .split("\n\n")
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let draws = input
        .get(0)
        .unwrap()
        .trim()
        .split(",")
        .map(|value| value.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut boards = input
        .iter()
        .skip(1)
        .map(|l| Board::new(l))
        .collect::<Vec<Board>>();

    let num_boards = boards.len();
    let mut won_boards = 0;
    'outer: for draw in draws {
        for board in &mut boards {
            board.mark_number_and_set_if_winner(draw);
            if board.winner {
                if won_boards == 0 {
                    println!("Part 1 Result: {}", board.sum_unselected * draw);
                }
                won_boards += 1;
                if won_boards == num_boards {
                    println!("Part 2 Result: {}", board.sum_unselected * draw);
                    break 'outer;
                }
            }
        }
        boards.retain(|board| !board.winner);
    }

    Ok(())
}
