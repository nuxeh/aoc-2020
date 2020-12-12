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

        let gen1: Vec<Vec<Cell>> = initial
            .windows(3)
            .enumerate()
            .map(|(l, v)| {
                izip!(&v[0], &v[1], &v[2])
                    .map(|(a, b, c)| vec![*a, *b, *c])
                    .collect::<Vec<Vec<Cell>>>()
                    .windows(3)
                    .enumerate()
                    .for_each(tick_window)
                    .collect()
            });

        loop {
            break;
        }
    }
}

fn tick_window(window: &[Vec<Cell>]) -> Cell {
    draw(window);

    let occupied = window
        .iter()
        .enumerate()
        .map(|(n, l)| {
            if n == 1 {
                l
                    .iter()
                    .enumerate()
                    .filter(|(n, _)| *n != 1)
                    .filter(|(_, s)| **s == Cell::OccupiedSeat)
                    .count()
            } else {
                l
                    .iter()
                    .filter(|v| **v == Cell::OccupiedSeat)
                    .count()
            }
        })
        .sum();

    println!("{}\n", occupied);

    let seat = window[1][1];

    let new_seat = match (seat, occupied) {
        (Cell::EmptySeat, 0) => Cell::OccupiedSeat,
        (Cell::OccupiedSeat, n) if n >= 4 => Cell::OccupiedSeat,
        _ => seat,
    };

    new_seat
}
