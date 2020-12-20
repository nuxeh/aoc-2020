use aocf::Aoc;
use std::collections::HashMap;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(20))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let tiles: HashMap<usize, Vec<Vec<bool>>> = i
            .lines()
            .fold((0usize, HashMap::new()), |mut acc, line| {
                if line.contains("Tile") {
                    acc.0 = line
                        .replace(":", "")
                        .split(" ")
                        .nth(1).unwrap()
                        .parse::<usize>().unwrap();
                } else {
                    let img_row: Vec<bool> = line
                        .chars()
                        .map(|c| c == '#')
                        .collect();

                    if let Some(t) = acc.1.get_mut::<usize>(&acc.0) {
                        t.push(img_row);
                    } else {
                        acc.1.insert(acc.0, vec![img_row]);
                    }
                }
                acc
            }).1;

        println!("{:?}", tiles);

    }
}
