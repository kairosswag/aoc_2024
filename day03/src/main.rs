use crate::State::*;
use std::io;
use std::io::BufRead;

pub fn main() {
    let stdin = io::stdin();
    let (p1, p2) = run(stdin.lock());

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

pub fn run<R>(mut reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut total_p1 = 0;
    let mut total_p2 = 0;
    let mut perform = true;
    let mut buf = Vec::new();
    while let Ok(len) = reader.read_to_end(&mut buf) {
        if len == 0 {
            break
        }
        for i in 0..len {
            let following = match buf[i] as char {
                'm' => try_for_mod(&buf[i..]),
                'd' => try_for_donots(&buf[i..]),
                _ => Boring,
            };

            match following {
                Boring => (),
                Do => perform = true,
                DoNot => perform = false,
                Mul(first, second) => {
                    let mul = first * second;
                    total_p1 += mul;
                    if perform {
                        total_p2 += mul;
                    }
                }
            }
        }
    }

    (total_p1, total_p2)
}

pub enum State {
    Boring,
    Do,
    DoNot,
    Mul(usize, usize),
}

fn try_for_mod(buf: &[u8]) -> State {
    if buf[0..4] == ['m' as u8, 'u' as u8, 'l' as u8, '(' as u8] {
        let mut curr_idx = 4;
        let mut first = Vec::new();
        while (buf[curr_idx] as char).is_digit(10) {
            first.push(buf[curr_idx] as char);
            curr_idx += 1;
        }
        if first.is_empty() {
            return Boring;
        }
        if buf[curr_idx] as char != ',' {
            return Boring;
        }
        curr_idx += 1;
        let mut second = Vec::new();
        while (buf[curr_idx] as char).is_digit(10) {
            second.push(buf[curr_idx] as char);
            curr_idx += 1;
        }
        if second.is_empty() {
            return Boring;
        }
        if buf[curr_idx] as char != ')' {
            return Boring;
        }

        Mul(
            first.iter().collect::<String>().parse::<usize>().unwrap(),
            second.iter().collect::<String>().parse::<usize>().unwrap(),
        )
    } else {
        Boring
    }
}

fn try_for_donots(buf: &[u8]) -> State {
    if buf[0..4] == ['d' as u8, 'o' as u8, '(' as u8, ')' as u8] {
        Do
    } else if buf[0..7] == ['d' as u8, 'o' as u8, 'n' as u8, '\'' as u8, 't' as u8, '(' as u8, ')' as u8] {
        DoNot
    } else {
        Boring
    }
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
