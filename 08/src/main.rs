use aocf::Aoc;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Cpu {
    pub pc: i32,
    pub acc: i32,
    mem: Vec<(Op, i32)>,
}

impl Cpu {
    fn new(mem: Vec<(Op, i32)>) -> Self {
        Self { pc: 0, acc: 0, mem }
    }

    fn exec(&mut self, ins: &(Op, i32)) {
        match ins.0 {
            Op::Acc => {
                self.acc += ins.1;
                self.pc += 1
            },
            Op::Jmp => self.pc += ins.1,
            Op::Nop => self.pc += 1,
            _ => (),
        }
    }

    fn tick(&mut self) -> Option<()> {
        // Eww
        let op = &self.clone().mem[self.pc as usize];
        self.exec(op);
        println!("{:?}", op);
        Some(())
    }
}

#[derive(Debug, Clone)]
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

        let mut cpu = Cpu::new(ins);
        let mut used: HashSet<i32> = HashSet::new();

        loop {
            cpu.tick();
            println!("pc {} acc {}", cpu.pc, cpu.acc);

            if used.contains(&cpu.pc) {
                break;
            } else {
                used.insert(cpu.pc);
            }
        }

    }
}
