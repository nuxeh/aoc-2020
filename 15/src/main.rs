use aocf::Aoc;
use std::collections::HashMap;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2020))
        .day(Some(15))
        .init()
        .unwrap();

    let input = aoc.get_input(false);

    if let Ok(i) = input {
        let starting_list: Vec<u32> = i
            .trim()
            .split(',')
            .map(|n| n.parse().ok())
            .flatten()
            .collect();

        println!("{:?}", starting_list);

        let mut map: HashMap<u32, (u32, Option<u32>, Option<u32>)> = HashMap::new();
        let mut last_spoken = None;

        for turn in 1..=2020 {
            let considered = if turn <= starting_list.len() {
                Some(starting_list[turn-1])
            } else {
                last_spoken
            };

            print!("[{}] {:?} ", considered.unwrap(), map);

            // Get record
            let (times_spoken, prev_turn, prev_prev_turn) = if map.contains_key(&considered.unwrap()) {
                *map.get(&considered.unwrap()).unwrap()
            } else {
                (0, None, None)
            };

            let spoken = if turn <= starting_list.len() {
                considered.unwrap()
            } else if times_spoken == 1 {
                0
            } else {
                prev_turn.unwrap() - prev_prev_turn.unwrap()
            };

            // Get record
            let (times_spoken, prev_turn, prev_prev_turn) = if map.contains_key(&spoken) {
                *map.get(&spoken).unwrap()
            } else {
                (0, None, None)
            };

            // Update record
            if map.contains_key(&spoken) {
                map.insert(spoken, (times_spoken + 1, Some(turn as u32), prev_turn));
            } else {
                map.insert(spoken, (1, Some(turn as u32), None));
            }

            println!("{}, ", spoken);
            last_spoken = Some(spoken);
        }
    }
}
