use aocf::Aoc;
use std::fmt;

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
                v[0].iter().zip(v[1].iter().zip(v[2].iter()))
                    .collect::<Vec<Vec<Cell>>>()
                    .windows(3)
                    .for_each(|w| println!("{:?}", w))
            });

        loop {
            break;
        }
    }
}

fn tick_generation() {

}
