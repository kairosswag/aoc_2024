use std::io;
use day05::run;
pub fn main() {
    let stdin = io::stdin();
    let (p1, p2) = run(stdin.lock());

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests_p5 {
    use crate::run;
    #[test]
    fn test() {
        let test_iput = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!((143, 123), run(test_iput.as_bytes()));
    }
}

/// v: 10, w: 20, x: 30, y: 40, z: 50 a: 90
/// v -> w -> x -> y -> z
/// a -> w
/// y -> a
///
///
#[test]
fn test_own() {
    let test_iput = "10|20
10|11
11|12
12|20
12|90
20|30
30|40
40|50
50|60
90|20
40|90
90|50

10,11,12,30,90,20,50
";
    assert_eq!((0, 90), run(test_iput.as_bytes()));
}

