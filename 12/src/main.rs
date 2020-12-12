use aocf::Aoc;

#[derive(Debug, Clone, Copy)]
struct Action {
    pub command: Command,
    pub param: i32,
}

impl Action {
    fn from_str(s: &str) -> Self {
        let mut chars = s.chars();
        Self {
            command: Command::from_char(chars.next().unwrap()),
            param: chars.collect::<String>().parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Command {
    None,
    N,
    S,
    E,
    W,
    L,
    R,
    F,
}

impl Command {
    fn from_char(c: char) -> Self {
        match c {
            'N' => Self::N,
            'S' => Self::S,
            'E' => Self::E,
            'W' => Self::W,
            'L' => Self::L,
            'R' => Self::R,
            'F' => Self::F,
            _ => Self::None,
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
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn move_towards(&mut self, dir: Direction, amount: i32) {
        match dir {
            Direction::North => self.y += amount,
            Direction::South => self.y -= amount,
            Direction::East => self.x += amount,
            Direction::West => self.x -= amount,
            _ => (),
        }
    }
}

impl Direction {
    fn from_degrees(d: i32) -> Self {
        match d {
            0 => Self::North,
            -270 | 90 => Self::East,
            -180 | 180 => Self::South,
            -90 | 270 => Self::West,
            _ => panic!(format!("bad degrees ({})", d)),
        }
    }

    fn get_degrees(&self) -> Option<i32> {
        match self {
            Self::North => Some(0),
            Self::East => Some(90),
            Self::South => Some(180),
            Self::West => Some(270),
            _ => panic!(format!("{:?}", self)),
        }
    }

    fn rotate(self, d: i32) -> Self {
        if let Some(deg) = self.get_degrees() {
            Self::from_degrees((deg + d) % 360)
        } else {
            panic!(format!("{:?} {}", self, d))
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Ship {
    pub coord: Coord,
    heading: Direction,
    waypoint: Waypoint,
}

impl Default for Ship {
    fn default() -> Self {
        Self { coord: Coord::new(0, 0), heading: Direction::East, waypoint: Waypoint::default() }
    }
}

impl Ship {
    fn exec(&mut self, a: Action) {
        let dir = match a.command {
            Command::N => Some(Direction::North),
            Command::E => Some(Direction::East),
            Command::S => Some(Direction::South),
            Command::W => Some(Direction::West),
            Command::F => Some(self.heading),
            Command::L => {
                self.heading = self.heading.rotate(-1 * a.param);
                None
            },
            Command::R => {
                self.heading = self.heading.rotate(a.param);
                None
            },
            _ => None,
        };

        if let Some(d) = dir {
            self.coord.move_towards(d, a.param);
        }
    }
}

use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
struct Waypoint {
    coord: Coord,
}

impl Waypoint {
    ///               | * (1, -3)
    ///               |
    /// (-3,-1) *     |
    ///         -------------
    ///               |     * (3, 1)
    ///               |
    ///      (-1,3) * |
    fn rotate(&mut self, d: i32) {
        let theta = (d as f32 / 360.0) * 2.0 * PI;
        match d {
            0 => (),
            180 => {
                self.coord.x = -1 * self.coord.x;
                self.coord.y = -1 * self.coord.y;
            },
            _ => {
                let sin = theta.sin() as i32;
                let cos = theta.cos() as i32;
                let new_x = (cos * self.coord.x) + (-1 * sin * self.coord.y);
                self.coord.y = (sin * self.coord.x) + (-1 * cos * self.coord.y);
                self.coord.x = new_x;
            },
        }
    }
}

impl Default for Waypoint {
    fn default() -> Self {
        Self { coord: Coord::new(0, 0) }
    }
}

#[cfg(test)]
#[test]
fn test_waypoint_rotate() {
    let mut wp = Waypoint { x: 1, y: -3};
    wp.rotate(90);
    assert_eq!(wp, Waypoint { x: 3, y: 1});
    wp.rotate(90);
    assert_eq!(wp, Waypoint { x: -1, y: 3});
    wp.rotate(90);
    assert_eq!(wp, Waypoint { x: -3, y: -1});
    wp.rotate(180);
    assert_eq!(wp, Waypoint { x: 3, y: 1});
    wp.rotate(270);
    assert_eq!(wp, Waypoint { x: 1, y: -3});
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
        let course = actions
            .iter()
            .map(|a| {
                ship.exec(*a);
                (a, ship)
            });

        course
            .for_each(|s| println!("{:?}", s));

        println!("{:#?}", ship);
        println!("{}", ship.coord.x.abs() + ship.coord.y.abs());
    }
}
