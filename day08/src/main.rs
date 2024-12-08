use std::io;
use day08::run;
pub fn main() {
    let stdin = io::stdin();
    let (p1, p2) = run(stdin.lock());

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests_p8 {
    use crate::run;
    #[test]
    fn test() {
        let test_iput = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!((14, 5), run(test_iput.as_bytes()));
    }

    #[test]
    fn test2() {
        let test_iput = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
..........CC
........A...
.........A..
..B.........
..B.........";
        assert_eq!((16, 5), run(test_iput.as_bytes()));
    }
}
