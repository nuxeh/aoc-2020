use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(1))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    //println!("{:?}", input);

    if let Ok(i) = input {
        //println!("{}", i);

        let ints = i.lines()
            .map(|s| s.parse().ok())
            .flatten()
            .collect::<Vec<u32>>();

        println!("{:?}", ints);

        // part 1
        for i in ints.clone() {
            for j in ints.clone() {
                if i + j == 2020 {
                    println!("{} + {} = 2020", i, j);
                    println!("{} x {} = {}", i, j, i * j);
                }
            }
        }

        // part 2
        for i in ints.clone() {
            for j in ints.clone() {
                for k in ints.clone() {
                    if i + j + k == 2020 {
                        println!("{} + {} + {} = 2020", i, j, k);
                        println!("{} x {} x {} = {}", i, j, k, i * j * k);
                    }
                }
            }
        }
    }
}
