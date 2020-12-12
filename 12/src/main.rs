use aocf::Aoc;

#[derive(Debug, Clone, Copy)]
struct Action {
    pub dir: Direction,
    pub param: i32,
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
    }

    fn from_degrees(d: i32) -> Self {
        match d {
            0 => Self::North,
            90 => Self::East,
            180 => Self::South,
            270 => Self::West,
            _ => Self::None,
        }
    }

    fn get_degrees(&self) -> Option<i32> {
        match self {
            Self::North => Some(0),
            Self::East => Some(90),
            Self::South => Some(180),
            Self::West => Some(270),
            _ => None,
        }
    }

    fn rotate(self, d: i32) -> Self {
        if let Some(deg) = self.get_degrees() {
            Self::from_degrees((deg + d) % 360)
        } else {
            Self::None
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
        let dir = match a.dir {
            Direction::North | Direction::South | Direction::East | Direction::West => a.dir,
            Direction::Forward => self.heading,
            Direction::Left => {
                self.heading = self.heading.rotate(-1 * a.param);
                Direction::None
            },
            Direction::Right => {
                self.heading = self.heading.rotate(a.param);
                Direction::None
            },
            _ => Direction::None,
        };

        match dir {
            Direction::North => self.y += a.param,
            Direction::South => self.y -= a.param,
            Direction::East => self.x += a.param,
            Direction::West => self.x -= a.param,
            _ => (),
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

        // Part 1
        let mut ship = Ship::default();
        actions
            .iter()
            .for_each(|a| ship.exec(*a));

        println!("{:#?}", actions);
        println!("{:#?}", ship);
        println!("{}", ship.x.abs() + ship.y.abs());
    }
}
