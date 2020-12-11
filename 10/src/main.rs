// TODO: revisit
use aocf::Aoc;
use std::collections::HashMap;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(10))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let mut joltage_adapters: Vec<u16> = i.lines()
            .map(|v| v.parse())
            .flatten()
            .collect();

        joltage_adapters.sort();

        println!("{:#?}", joltage_adapters);

        let differences: Vec<u16> = joltage_adapters
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect();

        let threes = differences
            .iter()
            .filter(|v| **v == 3)
            .count() + 1;
        let ones = differences
            .iter()
            .filter(|v| **v == 1)
            .count() + 1;

        println!("{:#?}", differences);
        println!("threes {} ones {} product {}", threes, ones, threes * ones);

        // Part 2
        part_2(joltage_adapters.as_slice());
    }
}

fn part_2(adapters: &[u16]) {
    let mut weights = HashMap::new();

    // Last first
    for n in adapters.iter().rev() {
        weights.insert(n, count(&adapters, &weights, *n));
    }

    println!("{}", weights.get(&0).unwrap());
}

fn count(adapters: &[u16], weights: &HashMap<&u16, u32>, n: u16) -> u32 {
    println!("{} {:?}", n, weights);

    if n == *adapters.last().unwrap() {
        return 1;
    }

    adapters
        .iter()
        .rev()
        .filter(|v| (**v as i32 - n as i32).abs() <= 3)
        .map(|v| {
            println!("{}", v);
            if let Some(w) = weights.get(v) {
                *w
            } else {
                count(adapters, weights, get_next(adapters, *v))
            }
        })
        .sum()
}

fn get_next(adapters: &[u16], n: u16) -> u16 {
    let next = adapters
        .iter()
        .skip_while(|v| **v != n)
        .skip(1)
        .nth(0)
        .unwrap();

    println!("{} -> {}", n, next);

    *next
}
