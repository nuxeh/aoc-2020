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
    if c == 'F' {
        println!("{:?}", (r.0, r.1 - r2));
        (r.0, r.1 - r2)
    } else {
        println!("{:?}", (r.0 + r2, r.1));
        (r.0 + r2, r.1)
    }
}

fn get_id(pass: &str) -> u32 {
    let r = pass.chars()
        .take(7)
        .fold((0, 127), scale);

    println!("{:?}", r);

    let c = pass.chars()
        .skip(7)
        .take(3)
        .fold((0, 7), scale);

    println!("{:?}", c);
    
    r.0 * 8 + c.0
}

#[cfg(test)]
#[test]
fn test_get_id() {
    assert_eq!(get_id("FBFBBFFRLR"), 357);
}
