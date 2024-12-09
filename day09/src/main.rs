use std::io;
use day09::run;
pub fn main() {
    let stdin = io::stdin();
    let (p1, p2) = run(stdin.lock());

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests_p9 {
    use crate::run;
    #[test]
    fn test() {
        let test_input = "2333133121414131402";
        assert_eq!((1928, 2858), run(test_input.as_bytes()));
    }
}
