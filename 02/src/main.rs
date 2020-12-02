use aocf::Aoc;

#[derive(Default, Debug)]
struct PassSpec {
    range_from: usize,
    range_to: usize,
    letter: String,
    password: String,
}

impl PassSpec {
    fn from_str(s: &str) -> Self {
        let mut r = Self::default();
        let parts: Vec<&str> = s.split(' ').collect();
        let range: Vec<usize> = parts[0].split('-').map(|s| s.parse()).flatten().collect();
        r.range_from = range[0];
        r.range_to = range[1];
        r.letter = parts[1].replace(":", "").to_string();
        r.password = parts[2].to_string();
        r
    }
}

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(2))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let v: Vec<PassSpec> = i.lines()
            .map(PassSpec::from_str)
            .collect();

        println!("{:?}", v);

        let mut valid = 0;

        for p in v {
            let count = p.password.split(&p.letter).count() - 1;
            if (p.range_from..p.range_to).contains(&count) {
                valid += 1;
            }
            println!("{} {} {} {}", p.password, p.letter, count, (p.range_from..p.range_to).contains(&count));
        }

        println!("valid count: {}", valid);
    }
}
