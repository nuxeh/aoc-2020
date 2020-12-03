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

    v.iter()
        .enumerate()
        .for_each(|(y, l)| l.chunks_exact(right)
                  .skip(y)
                  .take(1)
                  .for_each(|c| if c[0] {
                      trees += 1
                  }));

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

        //println!("{:#?}", v);
        
        let trees = count_trees(&i, 3, 1);

        println!("number of trees: {}", trees);

        for (r, d) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
            println!("{:?} {}", (r, d), count_trees(&i, *r, *d));
        }
    }
}
