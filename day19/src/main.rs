use std::io;
use day19::run;
pub fn main() {
    let stdin = io::stdin();
    let (p1, p2) = run(stdin.lock());

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests_day_19 {
    use crate::run;
    #[test]
    fn test() {
        let test_iput = "<fillme>";
        assert_eq!((5, 5), run(test_iput.as_bytes()));
    }
}
