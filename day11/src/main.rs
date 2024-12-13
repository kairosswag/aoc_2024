use std::io;
use day11::run;
pub fn main() {
    let stdin = io::stdin();
    let (p1, p2) = run(stdin.lock());

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests_p11 {
    use crate::run;
    #[test]
    fn test() {
        let test_input = "125 17";
        assert_eq!((55312, 5), run(test_input.as_bytes()));
    }
}
