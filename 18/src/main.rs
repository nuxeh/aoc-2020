use aocf::Aoc;

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

        let r: Vec<u32> = i
            .iter()
            .map(|l| {
                l
                    .split(' ')
                    .fold(vec![], |mut acc, c| {
                        match (c.parse::<u32>(), c) {
                            _ => acc.push(1u32),
                        };
                        acc
                    })[0]
            })
            .collect();

        println!("{:?}", r);

    }
}
