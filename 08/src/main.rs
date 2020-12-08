use aocf::Aoc;

struct Cpu {
    pc: usize,
    acc: i32,
}

impl Cpu {
    fn ins(&mut self, ins: &(Op, i32) {

    }
}

#[derive(Debug)]
enum Op {
    Acc,
    Jmp,
    Nop,
    None,
}

impl From<&str> for Op {
    fn from(item: &str) -> Self {
        match item {
            "acc" => Self::Acc,
            "jmp" => Self::Jmp,
            "nop" => Self::Nop,
            _ => Self::None,
        }
    }
}

/*
acc +17
jmp +1
nop +134
*/
fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(8))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let ins: Vec<(Op, i32)> = i.lines()
            .map(|v| v.split(' '))
            .map(|mut v| (v.next().unwrap().into(), v.next().unwrap().parse().unwrap()))
            .collect();

        println!("{:#?}", ins);

    }
}
