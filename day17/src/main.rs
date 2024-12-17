use std::io;
use day17::run;
pub fn main() {
    let stdin = io::stdin();
    let (p1, p2) = run(stdin.lock());

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests_day_17 {
    use crate::run;
    #[test]
    fn test() {
        let test_iput = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!(("4,6,3,5,6,3,5,2,1,0".to_string(), 5), run(test_iput.as_bytes()));
    }
}
