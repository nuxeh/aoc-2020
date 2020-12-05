use aocf::Aoc;
use std::collections::HashSet;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(5))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let passes = i.lines().map(get_id);

        // Part 1
        let max = passes.clone().max().unwrap();
        println!("{:?}", max);

        // Part 2
        let min = passes.clone().min().unwrap();
        let seats: HashSet<_> = (0..(127*8)).collect();
        let taken: HashSet<_> = passes.collect();
        let diff: Vec<&u32> = seats.difference(&taken).collect();

        diff.iter()
            .filter(|v| ***v > min && ***v < max)
            .for_each(|v| println!("{}", v));
    }
}

fn scale(r: (u32, u32), c: char) -> (u32, u32) {
    let r2 = (r.1 - r.0) / 2;
    match c {
        'F' => (r.0, r.1 - r2 - 1),
        'B' => (r.0 + r2 + 1, r.1),
        _ => r,
    }
}

fn get_id(pass: &str) -> u32 {
    let r = pass.chars()
        .take(7)
        .fold((0, 127), scale);

    let c = pass
        .replace("L", "F")
        .replace("R", "B")
        .chars()
        .skip(7)
        .take(3)
        .fold((0, 7), scale);

    r.0 * 8 + c.0
}

#[cfg(test)]
#[test]
fn test_get_id() {
    assert_eq!(get_id("FBFBBFFRLR"), 357);
    assert_eq!(get_id("FFFBBBFRRR"), 119);
    assert_eq!(get_id("BFFFBBFRRR"), 567);
    assert_eq!(get_id("BBFFBBFRLL"), 820);
}
