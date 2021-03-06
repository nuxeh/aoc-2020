use aocf::Aoc;
use std::collections::HashSet;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(6))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {

        // Part 1
        let group_answers = i.lines()
            .fold(vec![HashSet::new()], |mut acc, v| {
                if v.is_empty() {
                    acc.push(HashSet::new());
                    acc
                } else {
                    v
                        .chars()
                        .for_each(|f| {
                            acc.last_mut().unwrap().insert(f);
                        });
                    acc
                }
            });

        //println!("{:#?}", group_answers);

        let sums: Vec<_> = group_answers
            .iter()
            .map(|set| set.iter().count())
            .collect();

        println!("{}", sums.iter().sum::<usize>());

        // Part 2
        let group_answers: Vec<Vec<_>> = i.lines()
            .fold(vec![vec![]], |mut acc, v| {
                if v.is_empty() {
                    acc.push(vec![]);
                    acc
                } else {
                    let set: HashSet<_> = v.chars().collect();
                    acc.last_mut().unwrap().push(set);
                    acc
                }
            });

        //println!("{:#?}", group_answers);

        let group_intersections: Vec<_> = group_answers
            .iter()
            .map(|v| {
                let first = v.clone().into_iter().nth(0).unwrap();
                v
                    .iter()
                    .skip(1)
                    .fold(first, |acc, w| w.intersection(&acc).cloned().collect())
            })
            .collect();

        //println!("{:#?}", group_intersections);

        let result: usize = group_intersections
            .iter()
            .map(|v| v.iter().count())
            .sum();

        println!("{}", result);
    }
}
