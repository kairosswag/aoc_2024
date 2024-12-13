use std::io;
use day10::run;
pub fn main() {
    let stdin = io::stdin();
    let (p1, p2) = run(stdin.lock());

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests_p10 {
    use crate::run;
    #[test]
    fn test() {
        let test_input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!((36, 81), run(test_input.as_bytes()));
    }
}
