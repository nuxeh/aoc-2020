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

        println!("{:#?}", rules);

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

        part_1(&tickets, &rules);
    }
}

fn part_1(tickets: &Vec<Vec<u32>>, rules: &Vec<Vec<Vec<u32>>>) {
    let mut set: HashSet<u32> = HashSet::new();
    let num_rules = rules.iter().map(|s| s.iter().count()).sum::<usize>();
    println!("num rules: {}", num_rules);

    let res: Vec<u32> = tickets
        .iter()
        .skip(1) // your ticket
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
                        }

                        res
                })
                .max()
        })
        .flatten()
        .collect();

        println!("{:?}", res);
        println!("{:?}", set);
        println!("sum {}", set.iter().sum::<u32>());
}
