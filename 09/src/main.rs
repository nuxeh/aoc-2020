use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(9))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let numbers: Vec<u64> = i.lines()
            .map(|l| l.parse())
            .flatten()
            .collect();

        let n = 5;

        println!("{:#?}", numbers);

        let possible_sums = numbers
            .windows(n)
            .for_each(|v| println!("{:?}", v));

    }
}

fn is_sum(chunk: &[u64]) {

}
