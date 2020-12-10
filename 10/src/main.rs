use aocf::Aoc;

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

fn part_2(i: &[u16]) {
    println!("{}", count(i, 0, 0));
}

fn count(adapters: &[u16], n: u16, c: u32) -> u32 {
    let mut c = c;
    for a in adapters.iter().filter(|a| (**a as i16 - n as i16).abs() <= 3) {
        c += count(adapters, *a, c);
    }
    c
}
