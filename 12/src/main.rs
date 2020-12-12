use aocf::Aoc;

#[derive(Debug, Clone, Copy)]
struct Action {
    pub dir: Direction,
    pub param: u32,
}

impl Action {
    fn from_str(s: &str) -> Self {
        let mut chars = s.chars();
        Self {
            dir: Direction::from_char(chars.next().unwrap()),
            param: chars.collect::<String>().parse().unwrap(),
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

        fn degrees_left(&self, d: u32) -> Self {
            match (self, d {
                (Self::North, 90) => ,
                (Self::East, 90) => ,
                (Self::South, 90) => ,
                (Self::West, 90) => ,
                (Self::North, 180) => ,
                (Self::East, 180) => ,
                (Self::South, 180) => ,
                (Self::West, 180) => ,
                (Self::North, 270) => ,
                (Self::East, 270) => ,
                (Self::South, 270) => ,
                (Self::West, 270) => ,
            }
        }

        fn degrees_left(&self, d: u32) -> Self {
            match (self, d {
                (Self::North, 90) => ,
                (Self::East, 90) => ,
                (Self::South, 90) => ,
                (Self::West, 90) => ,
                (Self::North, 180) => ,
                (Self::East, 180) => ,
                (Self::South, 180) => ,
                (Self::West, 180) => ,
                (Self::North, 270) => ,
                (Self::East, 270) => ,
                (Self::South, 270) => ,
                (Self::West, 270) => ,
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Ship {
    pub x: i32,
    pub y: i32,
    heading: Direction,
}

impl Default for Ship {
    fn default() -> Self {
        Self { x:0, y:0, heading: Direction::East }
    }
}

impl Ship {
    fn exec(&mut self, a: Action) {
        match a.dir {
            Self::North => self.,
            Self::South => self.,
            Self::East => self.,
            Self::West => self.,
            Self::Left => self.,
            Self::Right => self.,
            Self::Forward => self.,
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
