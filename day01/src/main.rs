use itertools::Itertools;
use std::io;
use std::io::BufRead;

pub fn main() {
    let stdin = io::stdin();
    let (p1, p2) = run(stdin.lock());
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

pub fn run<R>(reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut list_one = Vec::new();
    let mut list_two = Vec::new();

    for line in reader.lines() {
        let line_buffer = line.expect("read error");
        let num_one = line_buffer[0..5].parse::<usize>().expect("parse error");
        let num_two = line_buffer[8..13].parse::<usize>().expect("parse error");
        list_one.push(num_one);
        list_two.push(num_two);
    }

    list_one.sort();
    list_two.sort();

    let counts_one = list_one.iter().counts();
    let counts_two = list_two.iter().counts();

    let mut total_p2 = 0;
    for (val, count_one) in counts_one {
        if let Some(count_two) = counts_two.get(&val) {
            total_p2 += val * count_one * count_two;
        }
    }

    let total_p1 = list_one
        .iter()
        .zip(list_two.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    (total_p1, total_p2)
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn web_example() {
        let input = b"00003   00004
00004   00003
00002   00005
00001   00003
00003   00009
00003   00003";
        assert_eq!((11, 31), run(&input[..]));
    }
}
