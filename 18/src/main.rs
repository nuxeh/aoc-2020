use aocf::Aoc;

#[derive(Debug)]
enum Op {
    Add,
    Mult,
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(18))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {

        let i: Vec<String> = i
            .lines()
            .map(|l| l.replace("(", "( "))
            .map(|l| l.replace(")", " )"))
            .collect();

        println!("{:?}", i);

        let r: Vec<Option<usize>> = i
            .iter()
            .map(|l| {
                l
                    .split(' ')
                    .fold((vec![None], vec![None]), |mut acc, c| {
                        match (c.parse::<usize>(), c) {
                            (_, "(") => {
                                acc.1.push(None);
                                acc.0.push(None);
                            },
                            (_, "*") => acc.1.push(Some(Op::Mult)),
                            (_, "+") => acc.1.push(Some(Op::Add)),
                            (_, ")") => {
                                match (acc.1.pop(), acc.0.pop(), acc.0.pop()) {
                                    (Some(Some(Op::Add)), Some(Some(l)), Some(Some(p))) => acc.0.push(Some(l + p)),
                                    (Some(Some(Op::Mult)), Some(Some(l)), Some(Some(p))) => acc.0.push(Some(l * p)),
                                    (Some(None), Some(Some(v)), _) => acc.0.push(Some(v)),
                                    (o, v1, v2) => {
                                        println!("{:?} {:?} {:?}", o, v1, v2)
                                    },
                                }
                            },
                            (Ok(n), _) => {
                                match (acc.1.pop(), acc.0.pop()) {
                                    (Some(None), _) => acc.0.push(Some(n)),
                                    (Some(Some(Op::Add)), Some(Some(l))) => acc.0.push(Some(l + n)),
                                    (Some(Some(Op::Mult)), Some(Some(l))) => acc.0.push(Some(l * n)),
                                    (a, b) => panic!("something's missing: ({:?}, {:?})", a, b),
                                }
                            },
                            _ => (),
                        };
                        println!("{} | {:?}", c, acc);
                        acc
                    }).0[0]
            })
            .collect();

        println!("{:?}", r);

        let s: usize = r
            .iter()
            .map(|v| v.unwrap())
            .sum();

        println!("{}", s);
    }
}
