use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(15))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let starting_list: Vec<u32> = i
            .trim()
            .split(',')
            .map(|n| n.parse().ok())
            .flatten()
            .collect();

        println!("{:?}", starting_list);

    }
}
