use aocf::Aoc;

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
            "jmp" => Self::Acc,
            "nop" => Self::Acc,
            _ => Self::None,
        }
    }
}

/*
acc +17
jmp +1
acc +16
acc +15
jmp +161
acc +37
acc +5
acc -13
nop +134
jmp +426
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
