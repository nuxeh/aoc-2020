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

        let possible_sums: Vec<Vec<u64>> = numbers
            .windows(n)
            .map(|v| {
                let first = v.get(0).unwrap();
                let mut sums = vec![];

                v
                    .iter()
                    .filter(|v| v != &first)
                    .for_each(|v| sums.push(v + first));

                sums
            })
            .collect();

        println!("{:#?}", possible_sums);
    }
}

fn is_sum(chunk: &[u64]) {

}
