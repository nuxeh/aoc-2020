// TODO: serde
use aocf::Aoc;

#[derive(Default)]
struct Ins {
    mask: Vec<Option<bool>>,
    sets: Vec<(u32, u32)>,
}

impl Ins {
    fn from_str(s: &str) -> Self {
        Self::default()
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
        let mut ins: Vec<Ins> = Vec::new();
        let iter = i.lines();

        iter
            .skip_while(|l| l.contains("mask"))
            .take(1)
            .for_each(|l| ins.push(Ins::from_str(l)));

        iter
            .take_while(|l| l.contains("mem"))
            .for_each(|l| {
                if let Some(m) = ins.last_mut() {
                    m.add_set_str(l);
                }
            });


        part_1();
    }
}

fn part_1() {
    let iv = vec![0; 36];

}
