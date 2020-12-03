use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(3))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let mut trees = 0;

        let v: Vec<Vec<bool>> = i.lines()
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();

        println!("{:#?}", v);

        println!("number of trees: {}", trees);
    }
}
