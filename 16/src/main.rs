use aocf::Aoc;
use std::collections::HashMap;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(16))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let rules: Vec<Vec<Vec<u32>>> = i.lines()
            .filter(|l| l.contains(':') && l.contains('-'))
            .map(|l| {
                l
                    .split(": ")
                    .filter(|s| s.contains('-'))
                    .collect::<String>()
                    .split(" or ")
                    .map(|r| {
                        r
                            .split('-')
                            .map(|v| v.parse())
                            .flatten()
                            .collect::<Vec<u32>>()
                    })
                    .collect()
            })
            .collect();

            println!("{:?}", rules);

    }
}
