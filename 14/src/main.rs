// TODO: serde
use aocf::Aoc;
use std::collections::HashMap;

#[derive(Debug, Default)]
struct Ins {
    mask: Vec<Option<bool>>,
    sets: Vec<(u32, u32)>,
}

impl Ins {
    fn from_str(s: &str) -> Self {
        let mut ret = Self::default();

        ret.mask = s
            .replace("mask = ", "")
            .chars()
            .map(|c| {
                match c {
                    'X' => None,
                    '1' => Some(true),
                    '0' => Some(false),
                    _ => panic!("bad char"),
                }
            })
            .collect();

        ret
    }

    fn add_set_str(&mut self, s: &str) -> &mut Self {
        let string = s
            .replace("mem[", "")
            .replace("] = ", ",");

        let mut nums = string
            .split(",")
            .take(2);

        let num_1 = nums.next().expect("get").parse::<u32>().expect("parse");
        let num_2 = nums.next().expect("get").parse::<u32>().expect("parse");

        self.sets.push((num_1, num_2));
        self
    }

    fn apply(&self, vec: &mut Vec<bool>) {
        let masked = self.sets
            .iter()
            .map(|s| {
                val_to_vec(s.1)
                    .iter()
                    .zip(self.mask.iter())
                    .enumerate()
                    .map(|(n, v)| {
                        match v {
                            (_, Some(true)) => vec[n] = true,
                            (_, Some(false)) => vec[n] = false,
                            (new_val, None) => vec[n] = *new_val,
                        }
                    })
            })
    }
}

fn val_to_vec(val: u32) -> Vec<bool> {
    format!("{:036b}", val)
        .chars()
        .map(|c| c == '1')
        .collect()
}

fn vec_to_val(vec: &[bool]) -> u32 {
    0

}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(14))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let ins = i
            .lines()
            .fold(vec![], |mut acc, l| {
                if l.contains("mask") {
                    acc.push(Ins::from_str(l));
                } else if l.contains("mem") {
                    if let Some(m) = acc.last_mut() {
                        m.add_set_str(l);
                    }
                };
                acc
            });

        println!("{:?}", ins);

        part_1();
    }
}

fn part_1() {
    let mem: HashMap<u32, u32> = HashMap::new();

}
