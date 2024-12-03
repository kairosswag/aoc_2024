use regex::Regex;
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
    let mut regex = Regex::new(r"mul\((?P<first>\d{1,3}),(?P<second>\d{1,3})\)").unwrap();
    let p1 = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            regex.captures_iter(&l).map(|val| {
                &val["first"].parse::<usize>().unwrap() * &val["second"].parse::<usize>().unwrap()
            }).sum::<usize>()
        })
        .sum();

    (p1, 5)
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn test() {
        assert_eq!(
            161,
            run(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
                    .as_bytes()
            )
            .0
        );
    }
}
