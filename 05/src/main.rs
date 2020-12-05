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
            let range = acc.1 - acc.0;
            if c == 'F' {
                (acc.0, (acc.1 / 2))
            } else {
                (acc.0 + (acc.1 / 2), acc.1)
            }
        });

    let c = pass.chars()
        .take(7)
        .fold((0, 127), |acc, c| {
            let range = acc.1 - acc.0;
            if c == 'F' {
                (acc.0, (acc.1 / 2))
            } else {
                (acc.0 + (acc.1 / 2), acc.1)
            }
        });
    
    r.0 * 8 + c.0
}

#[cfg(test)]
#[test]
fn test_get_id() {
    assert_eq!(get_id("FBFBBFFRLR"), 357);
}
