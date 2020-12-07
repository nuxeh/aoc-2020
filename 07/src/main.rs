use aocf::Aoc;
use std::hash::{Hash, Hasher};
use std::collections::{
    HashSet,
    HashMap,
    hash_map::DefaultHasher,
};
use rayon::prelude::*;

#[derive(Default, Debug)]
struct Bag {
    hash: u64,
    contents: Vec<u64>,
    weight: usize,
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(7))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        run(&i);
        run_2(&i);
    }
}

fn run(i: &str) {
    let bags: Vec<_> = i
        .lines()
        .map(parse)
        .collect();

    //println!("{:#?}", bags);

    let mut good_bags: HashSet<u64> = vec![hash_string("shiny gold")].into_iter().collect();
    let mut last_len = 1;

    loop {
        good_bags = {
            let mut new_good_bags = good_bags.clone();

            for good_bag in &good_bags {
                for bag in &bags {
                    if bag.contents.contains(&good_bag) {
                        new_good_bags.insert(bag.hash);
                    }
                }
            }

            new_good_bags
        };

        if good_bags.len() == last_len {
            break;
        }
        last_len = good_bags.len();

        println!("{}", good_bags.len() - 1);
    }
}

fn get_contained_bags(bags: &Vec<Bag>, bag: u64, set: &mut HashSet<u64>) {
    let bag = bags.iter().filter(|b| b.hash == bag).nth(0).unwrap();

    if !bag.contents.is_empty() {
        for bag in &bag.contents {
            set.insert(*bag);
            get_contained_bags(bags, *bag, set);
        }
    }

}

fn run_2(i: &str) {
    let mut bags: Vec<_> = i
        .lines()
        .map(parse)
        .collect();

    let mut weights: HashMap<_, _> = HashMap::new();
    let mut last_len = 1;

    let mut relevant_bags = HashSet::new();
    get_contained_bags(&bags, hash_string("shiny gold"), &mut relevant_bags);
    println!("{}", relevant_bags.iter().count());

    // Filter only relevant bags
    bags = bags
        .into_iter()
        .filter(|bag| relevant_bags.contains(&bag.hash))
        .collect();

    println!("{}", bags.iter().count());

    // Give root bags weights
    bags
        .iter_mut()
        .for_each(|bag| {
            if bag.contents.len() == 0 {
                bag.weight = 1;
                weights.insert(bag.hash, bag.weight);
            }
        });

    println!("{}", weights.iter().count());

    loop {
        println!("{:#?}", weights.iter().count());

        // Update weights for all bags and known weights
        for (hash, weight) in &weights {
            bags
                .iter_mut()
                .for_each(|bag| {
                    println!("==== {} ==== {} ====\n{:?}\n====", hash, weight, bag);
                    while bag.contents.contains(hash) {
                        println!("{:?}", bag.contents.binary_search(hash));
                        if let Ok(index) = bag.contents.binary_search(hash) {
                            println!("YES");
                            bag.weight += weight;
                            bag.contents.remove(index);
                        }
                    }
                });
        }

        // Update known weights
        bags
            .iter()
            .for_each(|bag| {
                if bag.contents.is_empty() && !weights.contains_key(&bag.hash) {
                    weights.insert(bag.hash, bag.weight);
                }
            });

        // End condition
        if weights.len() == last_len {
            break;
        }
        last_len = weights.len();

        println!("{:#?}", weights.iter().count());
    }

    //println!("{:#?}", bags);
    println!("{:#?}", weights.values().sum::<usize>() - 1);
}

#[cfg(test)]
#[test]
fn test_parse () {
    let demo = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    run(&demo);

    let demo2 = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
    run_2(&demo2);
}

fn parse(spec: &str) -> Bag {
    let mut bag = Bag::default();

    let parts: Vec<_> = spec.split(" bags contain ").collect();
    bag.hash = hash_string(parts.first().unwrap());
    let contents = parts.get(1).unwrap();

    if contents.contains("no other bags") {
        return bag;
    }

    let contents = contents.clone()
        .replace('.', "")
        .replace(" bags", "")
        .replace(" bag", "");

    let contents: Vec<_> = contents
        .split(", ")
        .map(parse_contents)
        .collect();

    contents
        .iter()
        .for_each(|c| {
            for _ in 0..c.0 {
                bag.contents.push(c.1);
            }
        });

    bag
}

fn parse_contents(spec: &str) -> (usize, u64) {
    let count: usize = spec.split(' ')
        .nth(0)
        .map(|v| v.parse().ok())
        .flatten()
        .unwrap();

    let tag: String = spec.split(' ')
        .skip(1)
        .map(|v| format!("{} ", v))
        .collect();

    let hash = hash_string(&tag.trim());

    (count, hash)
}

fn hash_string(text: &str) -> u64 {
    let mut s = DefaultHasher::new();
    text.hash(&mut s);
    s.finish()
}
