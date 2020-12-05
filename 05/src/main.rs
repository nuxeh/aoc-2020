use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(5))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        println!("{:?}", i.lines().map(get_id).max());
    }
}

fn scale(r: (u32, u32), c: char) -> (u32, u32) {
    let r2 = (r.1 - r.0) / 2;
    let ret = match c {
        'F' => (r.0, r.1 - r2 - 1),
        'B' => (r.0 + r2 + 1, r.1),
        _ => r,
    };
    println!("{} {} {:?}", c, r2, ret);
    ret
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

    println!("{} {} {} {}", pass, r.0, c.0, r.0 * 8 + c.0);

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
