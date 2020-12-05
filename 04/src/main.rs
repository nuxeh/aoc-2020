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
                    v.split(' ')
                        .for_each(|f| {
                            let mut s = f.split(':');
                            let key = s.next().unwrap();
                            let value = s.next().unwrap();
                            acc.last_mut().unwrap().insert(key, value);
                        });
                    acc
                }
            });

        //println!("{:#?}", passports);

        // Part 1
        valid += passports.iter()
            .filter(|p| p.len() == 8)
            .count();

        valid += passports.iter()
            .filter(|p| p.len() == 7 && !p.contains_key("cid"))
            .count();

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
   let a = map.get("byr")
        .map(|v| v.parse::<u32>().ok())
        .flatten()
        .filter(|v| *v >= 1920 && *v <= 2002);
   if a.is_none() { return false }

   let a = map.get("iyr")
        .map(|v| v.parse::<u32>().ok())
        .flatten()
        .filter(|v| *v >= 2010 && *v <= 2020);
   if a.is_none() { return false }

   let a = map.get("eyr")
        .map(|v| v.parse::<u32>().ok())
        .flatten()
        .filter(|v| *v >= 2020 && *v <= 2030);
   if a.is_none() { return false }

   let a = map.get("hgt")
        .filter(|v| {
            match (v.contains("in"), v.contains("cm"), v.replace("cm", "").replace("in", "").parse::<u32>()) {
                (true, _, Ok(num)) => num >= 59 && num <= 76,
                (_, true, Ok(num)) => num >= 150 && num <= 193,
                _ => false,
            }
        });
   if a.is_none() { return false }

   let a = map.get("hcl")
        .filter(|v| v.chars().take(1).collect::<String>() == "#");
   if a.is_none() { return false }

   let a = map.get("hcl")
        .map(|v| {
            "0x".chars()
                .chain(v.chars().skip(1))
                .collect::<String>()
                .parse::<u32>()
                .ok()
                .filter(|v| *v <= 0xffffff)
        });
   if a.is_none() { return false }

   let a = map.get("ecl")
        .filter(|v| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v));
   if a.is_none() { return false }

   let a = map.get("pid")
        .filter(|v| v.parse::<u32>().ok().is_some())
        .filter(|v| v.len() == 9);
   if a.is_none() { return false }

   true
}
