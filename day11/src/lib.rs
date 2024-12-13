use hashbrown::HashMap;
use std::io::BufRead;

pub fn run<R>(mut reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut initial: Vec<_> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|val| (val.parse::<usize>().unwrap(), 0))
        .collect();

    let mut input_p1 = initial.clone();
    let mut cache = HashMap::new();

    let mut stack = Vec::new();

    let mut result_p1 = 0;
    // always get the first one and go down
    let mut curr_layer = 0;
    let mut current = Vec::with_capacity(2);
    while let Some(stone) = input_p1.last() {
        let stone = stone.clone();
        if stone.1 != curr_layer {
            let value = stack.pop().unwrap();
            cache.insert(stone, value);
            input_p1.pop();
            if stone.1 == 0 {
                result_p1 += value;
            }
            curr_layer -= 1;
            continue;
        }
        if stone.1 == 25 {
            stack.iter_mut().for_each(|val| *val += 1);
            cache.insert(stone, 1);
            input_p1.pop();
            continue;
        }
        if stone.1 > 25 {
            panic!("wut?");
        }
        stack.push(0);
        current.clear();
        curr_layer += 1;
        let child_stones = put_resulting_stones(&stone, current);
        for child_stone in &child_stones {
            if let Some(cached) = cache.get(child_stone) {
                // do nothing
                stack.iter_mut().for_each(|val| *val += cached);
            } else {
                input_p1.push(child_stone.clone());
            }
        }
        current = child_stones;
    }

    let mut input_p2 = initial.clone();

    let mut cache = HashMap::new();

    let mut stack = Vec::new();

    let mut result_p2 = 0;
    // always get the first one and go down
    let mut curr_layer = 0;
    let mut current = Vec::with_capacity(2);
    while let Some(stone) = input_p2.last() {
        let stone = stone.clone();
        if stone.1 != curr_layer {
            let value = stack.pop().unwrap();
            cache.insert(stone, value);
            input_p2.pop();
            if stone.1 == 0 {
                result_p2 += value;
            }
            curr_layer -= 1;
            continue;
        }
        if stone.1 == 75 {
            stack.iter_mut().for_each(|val| *val += 1);
            cache.insert(stone, 1);
            input_p2.pop();
            continue;
        }
        if stone.1 > 75 {
            panic!("wut?");
        }
        stack.push(0);
        current.clear();
        curr_layer += 1;
        let child_stones = put_resulting_stones(&stone, current);
        for child_stone in &child_stones {
            if let Some(cached) = cache.get(child_stone) {
                // do nothing
                stack.iter_mut().for_each(|val| *val += cached);
            } else {
                input_p2.push(child_stone.clone());
            }
        }
        current = child_stones;
    }

    (result_p1, result_p2)
}

fn put_resulting_stones(
    stone: &(usize, usize),
    mut current: Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let next_layer = stone.1 + 1;
    if stone.0 == 0 {
        current.push((1, next_layer));
    } else {
        let digits = number_is_even(stone.0);
        if digits % 2 == 0 {
            let number_str = format!("{}", stone.0);
            let first = number_str[..digits / 2].parse().unwrap();
            let second = number_str[digits / 2..].parse().unwrap();
            current.push((first, next_layer));
            current.push((second, next_layer));
        } else {
            current.push((stone.0 * 2024, next_layer));
        }
    }

    current
}

fn number_is_even(number: usize) -> usize {
    if number < 10 {
        1
    } else if number < 000_000_000_000_100 {
        2
    } else if number < 000_000_000_001_000 {
        3
    } else if number < 000_000_000_010_000 {
        4
    } else if number < 000_000_000_100_000 {
        5
    } else if number < 000_000_001_000_000 {
        6
    } else if number < 000_000_010_000_000 {
        7
    } else if number < 000_000_100_000_000 {
        8
    } else if number < 000_001_000_000_000 {
        9
    } else if number < 000_010_000_000_000 {
        10
    } else if number < 000_100_000_000_000 {
        11
    } else if number < 001_000_000_000_000 {
        12
    } else if number < 010_000_000_000_000 {
        13
    } else if number < 100_000_000_000_000 {
        14
    } else {
        println!("number: {} ", number);
        panic!("Did not thing this would go that high");
    }
}
