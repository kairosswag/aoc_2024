use std::io;
use day06::run;
pub fn main() {
    let stdin = io::stdin();
    let (p1, p2) = run(stdin.lock());

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests_p6 {
    use crate::run;
    #[test]
    fn test() {
        let test_input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!((41, 6), run(test_input.as_bytes()));
    }
}
