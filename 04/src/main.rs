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

        //println!("{:#?}", i.lines().collect::<Vec<&str>>());

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

        valid += passports.iter()
            .filter(|p| p.len() == 8)
            .count();

        valid += passports.iter()
            .filter(|p| p.len() == 7 && !p.contains_key("cid"))
            .count();

        //println!("{:#?}", passports);
        println!("number of valid passports: {}", valid);

        // Part 2

        valid = passports.iter()
            .filter(|p| check_valid(p))
            .count();

        println!("number of valid passports: {}", valid);
    }
}

/*
• byr (Birth Year) - four digits; at least 1920 and at most 2002.
• iyr (Issue Year) - four digits; at least 2010 and at most 2020.
• eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
• hgt (Height) - a number followed by either cm or in:
    If cm, the number must be at least 150 and at most 193.
    If in, the number must be at least 59 and at most 76.
• hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
• ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
• pid (Passport ID) - a nine-digit number, including leading zeroes.
• cid (Country ID) - ignored, missing or not.
*/

fn check_valid(map: &HashMap<&str, &str>) -> bool {
   let a =  map.get("byr")
        .map(u32::parse())
        .filter(|v| v >= 1920 && w <= 2002);
   println!("{:#?}", a);
}
