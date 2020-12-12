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
    NoneDirection,
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
            _ => Self::NoneDirection,
        }
    }

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
            Direction::North | Direction::South | Direction::East | Direction::West => Some(a.dir),
            Direction::Forward => Some(self.heading),
            Direction::Left => {
                self.heading = self.heading.rotate(-1 * a.param);
                None
            },
            Direction::Right => {
                self.heading = self.heading.rotate(a.param);
                None
            },
            _ => None,
        };

        match dir {
            Some(Direction::North) => self.y += a.param,
            Some(Direction::South) => self.y -= a.param,
            Some(Direction::East) => self.x += a.param,
            Some(Direction::West) => self.x -= a.param,
            _ => (),
        }
    }
}

use std::f32::consts::PI;

#[derive(Debug, PartialEq)]
struct Waypoint {
    x: i32,
    y: i32,
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
                self.x = -1 * self.x;
                self.y = -1 * self.x;
            },
            _ => {
                println!("cos {} sin {}", theta.cos() as i32, theta.sin() as i32);
                let sin = theta.sin() as i32;
                let cos = theta.cos() as i32;
                let new_x = (cos * self.x) + (-1 * sin * self.y);
                self.y = (sin * self.x) + (-1 * cos * self.y);
                self.x = new_x;
            },
        }
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
        println!("{}", ship.x.abs() + ship.y.abs());
    }
}
