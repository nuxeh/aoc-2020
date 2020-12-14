use aocf::Aoc;
use std::collections::HashMap;

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

        //part_2(buses.as_slice());
        part_2_take_two(buses.as_slice());
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
    let num_buses = buses.len();
    let num_valid_buses = buses.iter().flatten().count();
    let largest_bus = buses.iter().flatten().max().unwrap();

    let start: usize = buses.iter().flatten().product::<usize>() / (num_buses * 2);
    let mut t: usize = (start / largest_bus) * largest_bus;

    println!("evaluating part 2.\nbus count: {} valid bus count: {}",
             num_buses, num_valid_buses);
    println!("largest bus: {}", largest_bus);
    println!("start at: {}", start);

    loop {
        let mut count = 0;

        for (n, u) in (t..(t+num_buses)).enumerate() {
            if let Some(b) = buses
                .iter()
                .nth(n)
                .map(|b| *b)
                .flatten() {
                    //println!("{} {} {}", b, u, u % b);
                    if u % b == 0 {
                        count += 1;
                    } else {
                        break;
                    }
                };
        }

        if t % 100000 == 0 {
            println!("{} {}", t, count);
        }

        if count == num_valid_buses {
            break;
        }

        t += largest_bus;
    }

    println!("{}", t);
}

fn part_2_take_two(buses: &[Option<usize>]) {
    let num_buses = buses.len();
    let largest_bus = buses.iter().flatten().max().unwrap();

    let mut factors: Vec<usize> = vec![0; buses.iter().flatten().count()];
    let num_factors = factors.len();

    factors[0] = 100000000000000 / largest_bus;

    //factors[0] = (100000000000000 / 23) - 1;

    let bus_offsets: Vec<_> = buses
        .iter()
        .enumerate()
        .map(|(n, v)| {
            v.map(|_| n)
        })
        .flatten()
        .collect();

    println!("{:?}", factors);
    println!("{:?}", bus_offsets);

    let mut buses: Vec<_> = buses.iter().flatten().collect();

    println!("{:?}", buses);

    let mut bus_offsets_map: HashMap<usize, usize> = HashMap::new();

    buses
        .iter()
        .zip(bus_offsets.iter())
        .for_each(|(b, o)| { bus_offsets_map.insert(**b, *o); () });

    buses.sort_by(|a, b| b.cmp(a));

    println!("{:?}", bus_offsets_map);
    println!("{:?}", buses);

    let mut factor = 1;

    loop {
        //println!("{:?}", factors);

        // Reached the end
        if factor == num_factors {
            break;
        }

        // Increment the factor
        factors[factor] += 1;

        // We're on the first factor, proceed to the next
        if factor == 0 {
            factor += 1;
            continue;
        }

        let cur_val = factors[factor] * buses[factor];
        let root_val = buses[0] * factors[0];
        let offset: usize = *bus_offsets_map.get(&buses[factor]).unwrap();
        let target_val = root_val + offset;

        // Gone too far, increase factors from root
        if cur_val > (root_val + num_buses) {
            //factors.iter_mut().skip(1).map(|v| *v = 0);
            factor = 0;
            continue;
        }

        // Matched target value, proceed to next factor
        if cur_val % target_val == 0 {
            factor += 1;

            if factor < num_factors {
                factors[factor] = root_val / buses[factor];
            }
        }
    }



    println!("{}", factors[0] * buses[0]);
}
