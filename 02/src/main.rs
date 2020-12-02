use aocf::Aoc;

#[derive(Default, Debug)]
struct PassSpec {
    range_from: u32,
    range_to: u32,
    letter: char,
    password: String,
}   

impl PassSpec {
    fn from_str(s: &str) -> Self {
        let mut r = Self::default();
        let parts: Vec<&str> = s.split(' ').collect();
        let range: Vec<u32> = parts[0].split('-').map(|s| s.parse().ok()).flatten().collect();
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
    }
}
