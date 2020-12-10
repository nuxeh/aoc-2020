use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(10))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let joltage_adapters: Vec<u16> = i.lines()
            .map(|v| v.parse())
            .flatten()
            .collect();

        println!("{:#?}", joltage_adapters);

    }
}
