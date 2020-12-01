use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(1))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    println!("{:?}", input);
}
