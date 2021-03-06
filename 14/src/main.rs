// TODO: serde
use aocf::Aoc;
use std::collections::HashMap;

#[derive(Debug, Default)]
struct Ins {
    mask: Vec<Option<bool>>,
    sets: Vec<(usize, usize)>,
    address_mask: Vec<Option<bool>>,
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

        let num_1 = nums.next().expect("get").parse::<usize>().expect("parse");
        let num_2 = nums.next().expect("get").parse::<usize>().expect("parse");

        self.sets.push((num_1, num_2));
        self
    }

    fn apply(&self, map: &mut HashMap<usize, usize>) {
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

    fn get_memory_addresses_for_mask(&self, mask: &[Option<bool>]) -> Vec<usize> {
        mask
            .iter()
            .enumerate()
            .fold(vec![vec![]], |mut acc, (n, v)| {
                match (n, v) {
                    (_, Some(val)) => {
                        acc
                            .iter_mut()
                            .for_each(|a| a.push(*val));
                        acc
                    },
                    // First needs special consideration, since it will have an empty vector
                    /*
                    (0, None) => {
                        println!("first");
                        //acc.push(vec![false]);
                        //acc.push(vec![true]);
                        println!("{:?}", acc);
                        acc
                    },
                    */
                    (_, None) => {
                        let mut new = acc.clone();
                        new
                            .iter_mut()
                            .for_each(|a| a.push(false));

                        let mut new_2 = acc.clone();
                        new_2
                            .iter_mut()
                            .for_each(|a| a.push(true));

                        new.extend_from_slice(&new_2);
                        new
                    },
                }
            })
            .iter()
            .map(|a| vec_to_val(a.as_slice()))
            .collect()
    }

    fn apply_2(&self, map: &mut HashMap<usize, usize>) {
        self.sets
            .iter()
            .for_each(|s| {
                let mask: Vec<Option<bool>> = val_to_vec(s.0)
                    .iter()
                    .zip(self.mask.iter())
                    .map(|v| {
                        match v {
                            (_, Some(true)) => Some(true),
                            (new_val, Some(false)) => Some(*new_val),
                            (_, None) => None,
                        }
                    })
                    .collect();

                //println!("{:?}", self.get_memory_addresses_for_mask(mask.as_slice()));

                self.get_memory_addresses_for_mask(mask.as_slice())
                    .iter()
                    .for_each(|a| { map.insert(*a as usize, s.1 as usize); () });

                //println!("{:#?}", map);
            });
    }
}

fn val_to_vec(val: usize) -> Vec<bool> {
    format!("{:036b}", val)
        .chars()
        .map(|c| c == '1')
        .collect()
}

fn vec_to_val(vec: &[bool]) -> usize {
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

    //println!("{}", string);
    usize::from_str_radix(&string, 2).unwrap()
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

        //println!("{:?}", ins);

        part_1(ins.as_slice());
        part_2(ins.as_slice());
    }
}

fn part_1(ins: &[Ins]) {
    let mut mem: HashMap<usize, usize> = HashMap::new();

    ins
        .iter()
        .for_each(|i| i.apply(&mut mem));

    println!("{:#?}", mem);

    let sum = mem
        .iter()
        .map(|(_, v)| v)
        .sum::<usize>();

    println!("sum {}", sum);
}

fn part_2(ins: &[Ins]) {
    let mut mem: HashMap<usize, usize> = HashMap::new();

    ins
        .iter()
        .for_each(|i| i.apply_2(&mut mem));

    println!("{:#?}", mem);

    let sum = mem
        .iter()
        .map(|(_, v)| v)
        .sum::<usize>();

    println!("sum {}", sum);

    let max = mem
        .keys()
        .max();

    println!("{:036b}", max.unwrap());
    // Hmm
    // 000011111100111001011110111111111101
}
