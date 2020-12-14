// TODO: serde
use aocf::Aoc;

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
        self.sets.push((1,1));
        self
    }
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
    let iv = vec![0; 36];

}
