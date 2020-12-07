use aocf::Aoc;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Default, Debug)]
struct Bag {
    hash: u64,
    contents: Vec<u64>,
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(7))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let bags: Vec<_> = i
            .lines()
            .map(parse)
            .collect();

        println!("{:#?}", bags);
    }
}

#[cfg(test)]
#[test]
fn test_parse () {
    let demo = "
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
    ";
}

fn parse(spec: &str) -> Bag {
    let mut bag = Bag::default();

    let parts: Vec<_> = spec.split(" bags contain ").collect();
    bag.hash = hash_string(parts.first().unwrap()); 
    let contents = parts.get(1).unwrap(); 

    bag
}

fn hash_string(text: &str) -> u64 {
    let mut s = DefaultHasher::new();
    text.hash(&mut s);
    s.finish()
}
