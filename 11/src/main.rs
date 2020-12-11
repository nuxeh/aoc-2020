use aocf::Aoc;
use std::fmt;
use itertools::izip;

#[derive(Debug, Clone, Copy)]
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

        initial
            .windows(3)
            .for_each(|v| {
                izip!(&v[0], &v[1], &v[2])
                    .collect::<Vec<(&Cell, &Cell, &Cell)>>()
                    .windows(3)
                    .for_each(|w| println!("{:?}", w))
            });

        loop {
            break;
        }
    }
}

fn tick_window(window: &[(&Cell, &Cell, &Cell)]) -> Vec<(Cell, Cell, Cell)> {
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
                l.iter().filter(|c| c == Cell::OccupiedSeat).count()
            }
        });

}
