use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(13))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let depart_at: u32 = i.lines().nth(0).unwrap().parse().unwrap();
        let buses: Vec<u32> = i.lines().nth(1).unwrap()
            .split(',')
            .filter(|i| !i.contains('x'))
            .map(|v| v.parse())
            .flatten()
            .collect();

        println!("departure {}", depart_at);
        println!("buses     {:?}", buses);

        part_1();
    }
}

fn part_1() {}
