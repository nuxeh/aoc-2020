use aocf::Aoc;
use std::fmt;
use std::iter;
use itertools::izip;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    None,
    Floor,
    EmptySeat,
    OccupiedSeat,
    Seen,
    Current,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Floor => write!(f, "{}", '.'),
            Self::EmptySeat => write!(f, "{}", 'L'),
            Self::OccupiedSeat => write!(f, "{}", '#'),
            Self::Seen => write!(f, "{}", '*'),
            Self::Current => write!(f, "{}", '@'),
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
        let line_length: usize = i.lines().nth(0).unwrap().len();
        let pad: String = iter::repeat('.').take(line_length).collect();

        let j = format!("{}\n{}{}", pad, i, pad);

        //println!("{}", i);
        //println!("{}", j);

        let initial: Vec<Vec<Cell>> = j.lines()
            .map(|l| {
                iter::once('.')
                    .chain(l.chars())
                    .chain(iter::once('.'))
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

        //println!("{:#?}", initial);
        draw(initial.as_slice());

        let mut cur_gen = initial.clone();
        let mut cur_occ = 0;

        loop {
            let mut new_gen = initial.clone();

            cur_gen
                .windows(3)
                .enumerate()
                .for_each(|(l, v)| {
                    izip!(&v[0], &v[1], &v[2])
                        .map(|(a, b, c)| vec![*a, *b, *c])
                        .collect::<Vec<Vec<Cell>>>()
                        .windows(3)
                        .enumerate()
                        .for_each(|(c, v)| {
                            new_gen[l+1][c+1] = evaluate_window(v);
                        })
                });

            draw(&new_gen);
            cur_gen = new_gen;

            let occupied: usize = cur_gen
                .iter()
                .map(|l| l.iter().filter(|s| **s == Cell::OccupiedSeat).count())
                .sum();

            if occupied == cur_occ {
                break;
            } else {
                cur_occ = occupied;
            }
        }

        println!("{}", cur_occ);

        part_2(&initial);
    }
}

fn evaluate_window(window: &[Vec<Cell>]) -> Cell {
    //draw(window);

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

    //println!("{}\n", occupied);

    let seat = window[1][1];

    let new_seat = match (seat, occupied) {
        (Cell::EmptySeat, 0) => Cell::OccupiedSeat,
        (Cell::OccupiedSeat, n) if n >= 4 => Cell::EmptySeat,
        _ => seat,
    };

    new_seat
}

fn part_2(initial: &Vec<Vec<Cell>>) {
    let mut cur_gen = initial.clone();
    let mut cur_occ = 0;
    let max_y = initial.iter().count();
    let max_x = initial.iter().nth(0).unwrap().len();

    loop {
        let mut new_gen = cur_gen.clone();

        for y in 0..max_y {
            for x in 0..max_x {
                let seat = cur_gen[y][x];
                let occupied = eval_cell_sightlines(&cur_gen, x, y, max_x - 1, max_y - 1);
                //println!("{}", occupied);
                //println!("{:?}", seat);

                let new_seat = match (seat, occupied) {
                    (Cell::EmptySeat, 0) => Cell::OccupiedSeat,
                    (Cell::OccupiedSeat, n) if n >= 5 => Cell::EmptySeat,
                    _ => seat,
                };

                new_gen[y][x] = new_seat;
            }
        }

        //draw(&new_gen);

        let occupied: usize = new_gen
            .iter()
            .map(|l| l.iter().filter(|s| **s == Cell::OccupiedSeat).count())
            .sum();

        if occupied == cur_occ {
            break;
        } else {
            cur_occ = occupied;
        }

        cur_gen = new_gen;
    }

    println!("{}", cur_occ);
}

fn eval_cell_sightlines(field: &Vec<Vec<Cell>>, x: usize, y: usize, max_x: usize, max_y: usize) -> usize {
    //println!("{} {}", x, y);
    let mut seen = field.clone();
    seen[y][x] = Cell::Current;

    let mut count = 0;

    let mut cur_x = x;
    count += loop {
        if cur_x == max_x { break 0 }
        cur_x += 1;
        if field[y][cur_x] == Cell::OccupiedSeat { break 1 }
        if field[y][cur_x] == Cell::EmptySeat { break 0 }
        seen[y][cur_x] = Cell::Seen;
    };

    let mut cur_x = x;
    count += loop {
        if cur_x == 0 { break 0}
        cur_x -= 1;
        if field[y][cur_x] == Cell::OccupiedSeat { break 1 }
        if field[y][cur_x] == Cell::EmptySeat { break 0 }
        seen[y][cur_x] = Cell::Seen;
    };

    let mut cur_y = y;
    count += loop {
        if cur_y == max_y { break 0}
        cur_y += 1;
        if field[cur_y][x] == Cell::OccupiedSeat { break 1 }
        if field[cur_y][x] == Cell::EmptySeat { break 0 }
        seen[cur_y][x] = Cell::Seen;
    };

    let mut cur_y = y;
    count += loop {
        if cur_y == 0 { break 0}
        cur_y -= 1;
        if field[cur_y][x] == Cell::OccupiedSeat { break 1 }
        if field[cur_y][x] == Cell::EmptySeat { break 0 }
        seen[cur_y][x] = Cell::Seen;
    };

    let mut cur_x = x;
    let mut cur_y = y;
    count += loop {
        if cur_x == max_x || cur_y == max_y { break 0}
        cur_x += 1;
        cur_y += 1;
        if field[cur_y][cur_x] == Cell::OccupiedSeat { break 1 }
        if field[cur_y][cur_x] == Cell::EmptySeat { break 0 }
        seen[cur_y][cur_x] = Cell::Seen;
    };

    let mut cur_x = x;
    let mut cur_y = y;
    count += loop {
        if cur_x == 0 || cur_y == 0 { break 0}
        cur_x -= 1;
        cur_y -= 1;
        if field[cur_y][cur_x] == Cell::OccupiedSeat { break 1 }
        if field[cur_y][cur_x] == Cell::EmptySeat { break 0 }
        seen[cur_y][cur_x] = Cell::Seen;
    };

    let mut cur_x = x;
    let mut cur_y = y;
    count += loop {
        if cur_x == max_x || cur_y == 0 { break 0}
        cur_x += 1;
        cur_y -= 1;
        if field[cur_y][cur_x] == Cell::OccupiedSeat { break 1 }
        if field[cur_y][cur_x] == Cell::EmptySeat { break 0 }
        seen[cur_y][cur_x] = Cell::Seen;
    };

    let mut cur_x = x;
    let mut cur_y = y;
    count += loop {
        if cur_x == 0 || cur_y == max_y { break 0}
        cur_x -= 1;
        cur_y += 1;
        if field[cur_y][cur_x] == Cell::OccupiedSeat { break 1 }
        if field[cur_y][cur_x] == Cell::EmptySeat { break 0 }
        seen[cur_y][cur_x] = Cell::Seen;
    };

    //draw(&seen);
    //println!("{}", count);

    count
}
