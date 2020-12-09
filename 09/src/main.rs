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

        let possible_sums: Vec<(u64, Vec<u64>)> = numbers
            .windows(n)
            .map(|v| {
                let first = v.get(0).unwrap();
                let mut sums = vec![];

                v
                    .iter()
                    .filter(|v| v != &first)
                    .for_each(|v| sums.push(v + first));

                (*first, sums)
            })
            .collect();

        possible_sums.iter().for_each(|(n, s)| println!("{} {:?}", n, s));

        possible_sums
            .windows(n+1)
            .map(|slice| {
                if let Some((c, _)) = slice.last() {
                    (*c, slice
                        .iter()
                        .take(n)
                        .filter(|v| v.1.contains(c))
                        .count())
                } else {
                    (0, 0)
                }
            })
            .filter(|c| c.1 == 0)
            .take(1)
            .for_each(|c| println!("{} {}", c.0, c.1));
    }
}
