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

    fn apply(&self, map: &mut HashMap<u32, u32>) {
        self.sets
            .iter()
            .for_each(|s| {
                let vec: Vec<_> = val_to_vec(s.1)
                    .iter()
                    .zip(self.mask.iter())
                    .map(|v| {
                        match v {
                            (_, Some(true)) => true,
                            (_, Some(false)) => false,
                            (new_val, None) => *new_val,
                        }
                    })
                    .collect();

                map.insert(s.0, vec_to_val(&vec));
            });
    }
}

fn val_to_vec(val: u32) -> Vec<bool> {
    format!("{:036b}", val)
        .chars()
        .map(|c| c == '1')
        .collect()
}

fn vec_to_val(vec: &[bool]) -> u32 {
    let string: String = vec
        .iter()
        .map(|v| {
            if *v {
                "1"
            } else {
                "0"
            }
        })
        .collect();

    u32::from_str_radix(&string, 2).unwrap()
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

        part_1(ins.as_slice());
    }
}

fn part_1(ins: &[Ins]) {
    let mut mem: HashMap<u32, u32> = HashMap::new();

    ins
        .iter()
        .for_each(|i| i.apply(&mut mem));

    println!("{:#?}", mem);
}