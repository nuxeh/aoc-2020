use aocf::Aoc;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(16))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let rules: Vec<Vec<Vec<u32>>> = i.lines()
            .filter(|l| l.contains(':') && l.contains('-'))
            .map(|l| {
                l
                    .split(": ")
                    .filter(|s| s.contains('-'))
                    .collect::<String>()
                    .split(" or ")
                    .map(|r| {
                        r
                            .split('-')
                            .map(|v| v.parse())
                            .flatten()
                            .collect::<Vec<u32>>()
                    })
                    .collect()
            })
            .collect();

        println!("{:?}", rules);

        let tickets: Vec<Vec<u32>> = i.lines()
            .filter(|l| l.contains(','))
            .map(|l| {
                l
                    .split(',')
                    .map(|v| v.parse())
                    .flatten()
                    .collect()
            })
            .collect();

        tickets
            .iter()
            .for_each(|t| println!("{:?}", t));

        let num_rules = rules.iter().map(|s| s.iter().count()).sum::<usize>();
        println!("number of rules: {}", num_rules);

        let ticket_failures = part_1(&tickets, &rules, num_rules);
        part_2(&tickets, &ticket_failures, &rules, num_rules);
    }
}

fn part_2(tickets: &Vec<Vec<u32>>, failures: &Vec<u32>, rules: &Vec<Vec<Vec<u32>>>, num_rules: usize) {
    // Remove non-matching tickets
    let tickets: Vec<Vec<u32>> = tickets
        .iter()
        .zip(failures.iter())
        .filter(|(_, f)| **f != num_rules as u32)
        .map(|(t, _)| t.clone())
        .collect();

    println!("{:?}", tickets);

    let candidate_fields: Vec<Vec<Vec<usize>>> = tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .map(|field| {
                    rules
                        .iter()
                        .enumerate()
                        .filter(|(_, ruleset)| {
                            (ruleset[0][0]..=ruleset[0][1]).contains(field) ||
                                (ruleset[1][0]..=ruleset[1][1]).contains(field)
                        })
                        .map(|(n, _)| n)
                        .collect::<Vec<usize>>()
                })
                .collect()
        })
        .collect();

        println!("{:?}", candidate_fields);
        candidate_fields
            .iter()
            .for_each(|f| println!("{:?}", f));

        let init_set: HashSet<usize> = (0..num_rules).collect();

        println!("{:?}", candidate_fields);
        let matches = candidate_fields
            .iter()
            .fold(vec![vec![init_set.clone(); 3]], |mut acc, t| {
                acc
                    .iter_mut()
                    .zip(t.iter())
                    .for_each(|(a, f)| {
                         f
                            .iter()
                            .filter(|c| !(0..num_rules).contains(c))
                            .for_each(|c| { a.remove(*c); () });
                    });
                acc
            });

        println!("{:?}", matches);
}

fn part_1(tickets: &Vec<Vec<u32>>, rules: &Vec<Vec<Vec<u32>>>, num_rules: usize) -> Vec<u32> {
    let mut set: HashSet<u32> = HashSet::new();
    let mut sum = 0;

    let res: Vec<u32> = tickets
        .iter()
        //.skip(1) // your ticket
        .map(|ticket| {
            ticket
                .iter()
                .map(|field| {
                    let res = rules
                        .iter()
                        .map(|ruleset| {
                            ruleset
                                .iter()
                                .map(|rule| (rule[0]..=rule[1]).contains(field))
                                .map(|res| {
                                    if res {
                                        0
                                    } else {
                                        1
                                    }
                                })
                                .sum::<u32>()
                        })
                        .sum::<u32>();

                        if res == num_rules as u32 {
                            set.insert(*field);
                            sum += field;
                        }

                        res
                })
                .max()
        })
        .flatten()
        .collect();

        println!("{:?}", res);
        println!("{:?}", set);
        println!("sum unique {}", set.iter().sum::<u32>());
        println!("sum non-unique {}", sum);

        res
}
