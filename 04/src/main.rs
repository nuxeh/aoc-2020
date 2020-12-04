use aocf::Aoc;
use std::collections::HashMap;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(4))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let mut valid = 0;

        println!("{:#?}", i.lines().collect::<Vec<&str>>());

        let passports = i.lines()
            .fold(vec![HashMap::new()], |mut acc, v| {
                if v.is_empty() {
                    acc.push(HashMap::new());
                    acc
                } else {
                    v.split(" ")
                        .for_each(|f| {
                            let mut s = f.split(":");
                            let key = s.next().unwrap();
                            let value = s.next().unwrap();
                            acc.last_mut().unwrap().insert(key, value);
                        });
                    acc
                }
            });

        println!("{:#?}", passports);

        println!("number of valid passports: {}", valid);

    }
}
