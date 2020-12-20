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
            .filter(|v| !v.is_empty())
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

        part_1(&tiles);
    }
}

fn part_1(tiles: &HashMap<usize, Vec<Vec<bool>>>) {
    let mut edge_vals: HashMap<usize, Vec<usize>> = HashMap::new();

    tiles
        .iter()
        .for_each(|(k, v)| {
            let top = v.first().unwrap();
            let top = top.iter().map(|col| if *col { "1" } else { "0" }).collect::<String>();
            let top = usize::from_str_radix(&top, 2).unwrap();

            let btm = v.last().unwrap();
            let btm = btm.iter().map(|col| if *col { "1" } else { "0" }).collect::<String>();
            let btm = usize::from_str_radix(&btm, 2).unwrap();

            let lft = v
                .iter()
                .map(|row| if *row.first().unwrap() { "1" } else { "0" })
                .fold(String::new(), |acc, v| format!("{}{}", acc, v));
            let lft = usize::from_str_radix(&lft, 2).unwrap();

            let rgt = v
                .iter()
                .map(|row| if *row.last().unwrap() { "1" } else { "0" })
                .fold(String::new(), |acc, v| format!("{}{}", acc, v));
            let rgt = usize::from_str_radix(&rgt, 2).unwrap();

            edge_vals.insert(*k, vec![top, btm, lft, rgt]);
        });

    println!("{:?}", edge_vals);
}
