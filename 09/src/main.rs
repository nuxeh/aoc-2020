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

        println!("{:#?}", numbers.chunks_exact(25));

        let prev_25: &[u64] = numbers
            .iter()
            .skip(0)
            .take(25)
            .collect();

    }
}

fn is_sum(chunk: &[u64]) {

}
