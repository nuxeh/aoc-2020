use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(13))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let depart_at: u32 = i.lines().nth(0).unwrap().parse().unwrap();
        let buses: Vec<u32> = i.lines().nth(1).unwrap()
            .split(',')
            .filter(|i| !i.contains('x'))
            .map(|v| v.parse())
            .flatten()
            .collect();

        println!("departure: {}", depart_at);
        println!("buses:     {:?}", buses);

        part_1(depart_at, buses.as_slice());

        let buses: Vec<Option<usize>> = i.lines().nth(1).unwrap()
            .split(',')
            .map(|v| v.parse().ok())
            .collect();

        println!("{:?}", buses);

        part_2(buses.as_slice());
    }
}

fn part_1(dep: u32, buses: &[u32]) {
    for t in dep..dep+40 {
        print!("{} | ", t);
        buses
            .iter()
            .filter(|b| t % **b == 0)
            .for_each(|b| print!("{} ({}) ", b, b * (t - dep)));
        print!("\n");
    }
}

fn part_2(buses: &[Option<usize>]) {
    let mut t: usize = 0;
    let num_buses = buses.len();

    loop {
        for (n, u) in (t..(t+num_buses)).enumerate() {
            if let Some(b) = buses
                .iter()
                .nth(n)
                .map(|b| *b)
                .flatten()
                .filter(|b| u % b == 0) {
                    println!("{} {}", b, t);
                };
        }

        t += 1;
    }
}
