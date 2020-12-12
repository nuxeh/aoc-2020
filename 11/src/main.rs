use aocf::Aoc;
use std::fmt;
use itertools::izip;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    None,
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Floor => write!(f, "{}", '.'),
            Self::EmptySeat => write!(f, "{}", 'L'),
            Self::OccupiedSeat => write!(f, "{}", '#'),
            Self::None => write!(f, "{}", '!'),
        }
    }
}

fn draw(field: &[Vec<Cell>]) {
   field
       .iter()
       .for_each(|r| {
           r
               .iter()
               .for_each(|c| print!("{}", c));
           print!("\n");
       });
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(11))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let initial: Vec<Vec<Cell>> = i.lines()
            .map(|l| {
                l.chars()
                    .map(|c| {
                        match c {
                            '.' => Cell::Floor,
                            'L' => Cell::EmptySeat,
                            '#' => Cell::OccupiedSeat,
                            _ => Cell::None,
                        }
                    })
                .collect()
            })
            .collect();

        println!("{:#?}", initial);
        draw(initial.as_slice());

        let gen1: Vec<Vec<(Cell, Cell, Cell)>> = initial
            .windows(3)
            .map(|v| {
                izip!(&v[0], &v[1], &v[2])
                    .collect::<Vec<(&Cell, &Cell, &Cell)>>()
                    .windows(3)
                    .map(tick_window)
                    .collect::<Vec<(Cell, Cell, Cell)>>()
            })
            .collect();

        loop {
            break;
        }
    }
}

fn tick_window<'a>(window: &'a [(&'a Cell, &'a Cell, &'a Cell)]) -> Vec<(Cell, Cell, Cell)> {
    let occupied = window
        .iter()
        .enumerate()
        .map(|(n, l)| {
            if n == 1 {
                match l {
                    (Cell::OccupiedSeat, _, Cell::OccupiedSeat) => 2,
                    (Cell::OccupiedSeat, _, Cell::EmptySeat) => 1,
                    (Cell::EmptySeat, _, Cell::OccupiedSeat) => 1,
                    _ => 0,
                }
            } else {
                if let (a, b, c) = l {
                    let mut count = 0;
                    if **a == Cell::OccupiedSeat {
                        count += 1;
                    }
                    if **b == Cell::OccupiedSeat {
                        count += 1;
                    }
                    if **c == Cell::OccupiedSeat {
                        count += 1;
                    }
                    count
                } else {
                    0
                }
            }
        })
        .sum();

    let seat = window[1].1;

    let new_seat = match (seat, occupied) {
        (Cell::EmptySeat, 0) => Cell::OccupiedSeat,
        (Cell::OccupiedSeat, n) if n >= 4 => Cell::OccupiedSeat,
        _ => *seat,
    };

    let mut new_window: Vec<(Cell, Cell, Cell)> = window.clone()
        .iter()
        .map(|v| (*v.0, *v.1, *v.2))
        .collect();

    new_window[1].1 = new_seat;

    new_window
}
