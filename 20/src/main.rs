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
        let mut tiles: HashMap<usize, Vec<Vec<bool>>> = HashMap::new();

        i
            .lines()
            .fold(0usize, |mut acc, line| {
                if line.contains("Tile") {
                    acc = line
                        .replace(":", "")
                        .split(" ")
                        .nth(1).unwrap()
                        .parse::<usize>().unwrap();
                } else {
                    let img_row: Vec<bool> = line
                        .chars()
                        .map(|c| c == '#')
                        .collect();

                    if let Some(t) = tiles.get_mut(&acc) {
                        t.push(img_row);
                    } else {
                        tiles.insert(acc, vec![img_row]);
                    }
                }
                acc
            });

        println!("{:?}", tiles);

    }
}
