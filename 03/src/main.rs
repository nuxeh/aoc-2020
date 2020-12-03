use aocf::Aoc;

fn count_trees(i: &str, right: usize, down: usize) -> usize {
    let mut trees = 0;

    let v: Vec<Vec<bool>> = i
        .repeat(down)
        .lines()
        .map(|l| {
            l.repeat(200).chars().map(|c| c == '#').collect()
        })
        .collect();

    v.chunks_exact(down)
        .enumerate()
        .for_each(|(y, c)| c.iter()
             .take(1)
             .for_each(|l| l.chunks_exact(right)
                    .skip(y)
                    .take(1)
                    .for_each(|c| if c[0] {
                        trees += 1
                    })));
    trees
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(3))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {

        println!("{}", i.repeat(2));
        
        let trees = count_trees(&i, 3, 1);

        println!("number of trees: {}", trees);

        let mut multiplicand = 1;

        for (r, d) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
            let trees = count_trees(&i, *r, *d);
            println!("{:?} {}", (r, d), trees);
            multiplicand *= trees;
        }

        println!("number of trees: {}", multiplicand);

    }
}
