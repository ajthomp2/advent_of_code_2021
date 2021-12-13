use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Coord(usize, usize);

#[derive(Debug)]
struct Line(Coord, Coord);

impl Line {
    fn from_input(input: &str) -> Self {
        let (left, right) = input.split_once(" -> ").unwrap();
        let (left_x, left_y) = left.split_once(",").unwrap();
        let (right_x, right_y) = right.split_once(",").unwrap();
        Line(
            Coord(
                left_x.parse::<usize>().unwrap(),
                left_y.parse::<usize>().unwrap(),
            ),
            Coord(
                right_x.parse::<usize>().unwrap(),
                right_y.parse::<usize>().unwrap(),
            ),
        )
    }

    fn is_vert(&self) -> bool {
        self.0 .0 == self.1 .0
    }

    fn is_horz(&self) -> bool {
        self.0 .1 == self.1 .1
    }
}

impl<'a> IntoIterator for &'a Line {
    type Item = Coord;
    type IntoIter = LineIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LineIterator {
            line: self,
            curr_pos: Coord(self.0 .0, self.0 .1),
            done: false,
        }
    }
}

struct LineIterator<'a> {
    line: &'a Line,
    curr_pos: Coord,
    done: bool,
}

impl<'a> Iterator for LineIterator<'a> {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_pos == self.line.1 {
            if self.done {
                return None;
            }
            self.done = true;
            return Some(Coord(self.curr_pos.0, self.curr_pos.1));
        }
        let next_x = if self.curr_pos.0 < self.line.1 .0 {
            self.curr_pos.0 + 1
        } else if self.curr_pos.0 > self.line.1 .0 {
            self.curr_pos.0 - 1
        } else {
            self.curr_pos.0
        };
        let next_y = if self.curr_pos.1 < self.line.1 .1 {
            self.curr_pos.1 + 1
        } else if self.curr_pos.1 > self.line.1 .1 {
            self.curr_pos.1 - 1
        } else {
            self.curr_pos.1
        };
        let return_val = Coord(self.curr_pos.0, self.curr_pos.1);
        self.curr_pos = Coord(next_x, next_y);
        Some(return_val)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let coords = fs::read_to_string("src/data/day5_input.txt")?
        .trim()
        .split("\n")
        .map(|l| Line::from_input(l))
        .collect::<Vec<Line>>();

    let mut coord_counts = HashMap::<Coord, usize>::new();
    let mut num_horz_or_vert_overlaps = 0;
    for line in &coords {
        if line.is_vert() || line.is_horz() {
            for coord in line {
                update_counts(coord, &mut coord_counts, &mut num_horz_or_vert_overlaps);
            }
        }
    }

    println!("Part 1 Result: {}", num_horz_or_vert_overlaps);

    let mut num_overlaps = num_horz_or_vert_overlaps;
    for line in &coords {
        if !line.is_vert() && !line.is_horz() {
            for coord in line {
                update_counts(coord, &mut coord_counts, &mut num_overlaps);
            }
        }
    }

    println!("Part 2 Result: {}", num_overlaps);

    Ok(())
}

fn update_counts(coord: Coord, coord_counts: &mut HashMap<Coord, usize>, num_overlaps: &mut usize) {
    if let Some(count) = coord_counts.get_mut(&coord) {
        if *count == 1 {
            *num_overlaps += 1;
        }
        *count += 1;
    } else {
        coord_counts.insert(coord, 1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vert_iterator() {
        let line = Line(Coord(1, 1), Coord(1, 3));
        let mut iter = line.into_iter();
        assert_eq!(iter.next(), Some(Coord(1, 1)));
        assert_eq!(iter.next(), Some(Coord(1, 2)));
        assert_eq!(iter.next(), Some(Coord(1, 3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_horz_iterator() {
        let line = Line(Coord(1, 1), Coord(3, 1));
        let mut iter = line.into_iter();
        assert_eq!(iter.next(), Some(Coord(1, 1)));
        assert_eq!(iter.next(), Some(Coord(2, 1)));
        assert_eq!(iter.next(), Some(Coord(3, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_diag_iterator_1() {
        let line = Line(Coord(1, 1), Coord(3, 3));
        let mut iter = line.into_iter();
        assert_eq!(iter.next(), Some(Coord(1, 1)));
        assert_eq!(iter.next(), Some(Coord(2, 2)));
        assert_eq!(iter.next(), Some(Coord(3, 3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_diag_iterator_2() {
        let line = Line(Coord(9, 7), Coord(7, 9));
        let mut iter = line.into_iter();
        assert_eq!(iter.next(), Some(Coord(9, 7)));
        assert_eq!(iter.next(), Some(Coord(8, 8)));
        assert_eq!(iter.next(), Some(Coord(7, 9)));
        assert_eq!(iter.next(), None);
    }
}
