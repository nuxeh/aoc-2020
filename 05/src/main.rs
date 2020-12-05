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

fn get_id(pass: &str) -> u32 {
    let r = pass.chars()
        .take(7)
        .fold((0, 127), |acc, c| {
            let r2 = (acc.1 - acc.0) / 2;
            if c == 'F' {
                (acc.0, acc.1 - r2)
            } else {
                (acc.0 + r2, acc.1)
            }
        });

    println!("{:?}", r);

    let c = pass.chars()
        .skip(7)
        .take(3)
        .fold((0, 127), |acc, c| {
        });
    
    r.0 * 8 + c.0
}

#[cfg(test)]
#[test]
fn test_get_id() {
    assert_eq!(get_id("FBFBBFFRLR"), 357);
}
