use hashbrown::HashMap;
use std::io::BufRead;
use itertools::Itertools;

pub fn run<R>(reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let initial: Vec<_> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|val| (val.parse::<usize>().unwrap(), 0))
        .collect();

    let (result_p1, result_p2) = solve_iterative(initial.clone());

    (result_p1, result_p2)
}

fn solve_iterative(input: Vec<(usize, usize)>) -> (usize, usize) {
    let mut working = HashMap::from_iter(input.iter().map(|val| val.0).counts());
    let mut next = HashMap::new();

    let mut res_1 = 0;
    for blink in 0..75 {
        for (stone_val, stone_count) in &working {
            let (stone_val, stone_count) = (*stone_val, *stone_count);
            if stone_val == 0 {
                next.entry(1).and_modify(|curr_count| *curr_count += stone_count).or_insert(stone_count);
            } else {
                let digits = number_is_even(stone_val);
                if digits % 2 == 0 {
                    let pivot = 10usize.pow((digits / 2) as u32);
                    let first = stone_val / pivot;
                    let second = stone_val % pivot;
                    next.entry(first).and_modify(|curr_count| *curr_count += stone_count).or_insert(stone_count);
                    next.entry(second).and_modify(|curr_count| *curr_count += stone_count).or_insert(stone_count);
                } else {
                    next.entry(stone_val * 2024).and_modify(|curr_count| *curr_count += stone_count).or_insert(stone_count);
                }
            }
        }
        if blink == 24 {
            res_1 = next.values().sum()
        }
        working = next;
        next = HashMap::new();
    }

    (res_1, working.values().sum())
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

#[test]
fn test_format_stuff() {
    let number = 123456usize;
    let length = 6;
    let pivot = 10usize.pow(length / 2);
    let lower = number / pivot;
    let upper = number % pivot;
    println!("{lower}, {upper}");
}
