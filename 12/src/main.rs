use aocf::Aoc;

#[derive(Debug, Clone, Copy)]
struct Action {
    dir: Direction,
    dist: u32,
}

impl Action {
    fn from_str(s: &str) -> Self {
        let mut chars = s.chars();
        Self {
            dir: Direction::from_char(chars.next().unwrap()),
            dist: chars.collect::<String>().parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    None,
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'N' => Self::North,
            'S' => Self::South,
            'E' => Self::East,
            'W' => Self::West,
            'L' => Self::Left,
            'R' => Self::Right,
            'F' => Self::Forward,
            _ => Self::None,
        }
    }
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(12))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let actions: Vec<_> = i
            .lines()
            .map(Action::from_str)
            .collect();

        println!("{:#?}", actions);
    }
}
