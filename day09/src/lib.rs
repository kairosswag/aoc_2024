use hashbrown::{HashMap, HashSet};
use std::io::BufRead;

pub fn run<R>(mut reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut buf_p1 = [0u8; 2usize.pow(16)];
    let read = reader.read(&mut buf_p1).unwrap();

    let res_p2 = solve_p2(&buf_p1[..read], read); // p2 first since it won't mut the array
    let res_p1 = solve_p1(&mut buf_p1, read);

    (res_p1, res_p2)
}

fn solve_p2(buf: &[u8], read: usize) -> usize {
    let values_back: Vec<(usize, u8)> = buf
        .iter()
        .enumerate()
        .filter(|(line, _val)| line % 2 == 0)
        .map(|(a, b)| (a / 2, *b - b'0'))
        .rev()
        .collect();

    let mut hole_indices = Vec::new();
    for (hole_pos, val) in buf.iter().enumerate() {
        if hole_pos % 2 == 1 {
            let val = *val - b'0';
            hole_indices.push((hole_pos, val));
        }
    }

    let mut in_holes = HashMap::new();
    let mut moved = HashSet::new();
    'outer: for (val_idx, val_len) in values_back {
        for idx in 0..hole_indices.len() {
            let (hole_start, hole_len) = hole_indices[idx];
            if hole_len >= val_len && (hole_start / 2) < val_idx {
                in_holes.entry(hole_start).or_insert(Vec::new()).push((val_idx, val_len));
                moved.insert(val_idx);
                hole_indices[idx].1 = hole_len - val_len;
                continue 'outer;
            }
        }
    }


    let mut res_p2 = 0;
    let mut gen_idx = 0;
    for index in 0..read - 1 {
        if index % 2 == 0 {
            let value = buf[index] - b'0';
            if !moved.contains(&(index/2)) {
                for _ in 0..value {
                    res_p2 += gen_idx * (index / 2);
                    gen_idx += 1;
                }
            } else {
                for _ in 0..value {
                }
                gen_idx += value as usize;
            }
        } else {
            if let Some(fillers) = in_holes.get(&index) {
                for (filler_idx, filler_len) in fillers {
                    for _ in 0..*filler_len {
                        res_p2 += gen_idx * filler_idx;
                        gen_idx += 1;
                    }
                }
            }
            gen_idx += hole_indices[(index - 1) / 2].1 as usize;
        }
    }

    res_p2
}

fn solve_p1(mut buf: &mut [u8], read: usize) -> usize {
    let mut res_p1 = 0;
    let mut back_idx = read - 1;
    let mut array_idx = 0;
    let mut gen_idx: usize = 0;
    let mut even = true;
    while array_idx < back_idx {
        if even {
            for _ in 0..(buf[array_idx] - b'0') {
                res_p1 += gen_idx * (array_idx / 2);
                gen_idx += 1;
            }
        } else {
            fill_remaining(
                &mut res_p1,
                &mut buf,
                array_idx,
                &mut gen_idx,
                &mut back_idx,
            );
        }
        array_idx += 1;
        even = !even;
    }
    if buf[back_idx] > 0 {
        for _ in 0..(buf[back_idx] - b'0') {
            res_p1 += gen_idx * (back_idx / 2);
            gen_idx += 1;
        }
    }
    res_p1
}

fn fill_remaining(
    res_p1: &mut usize,
    buf: &mut [u8],
    array_idx: usize,
    gen_idx: &mut usize,
    back_idx: &mut usize,
) {
    let spots = (buf[array_idx] - b'0') as usize;
    let remaining = (buf[*back_idx] - b'0') as usize;
    if remaining == spots {
        for _ in 0..remaining {
            *res_p1 += *gen_idx * (*back_idx / 2);
            *gen_idx += 1;
        }
        *back_idx -= 2;
    } else if remaining > spots {
        for _ in 0..spots {
            *res_p1 += *gen_idx * (*back_idx / 2);
            *gen_idx += 1;
        }
        buf[*back_idx] = (remaining - spots) as u8 + b'0';
    } else if remaining < spots {
        for _ in 0..remaining {
            *res_p1 += *gen_idx * (*back_idx / 2);
            *gen_idx += 1;
        }
        buf[array_idx] = (spots - remaining) as u8 + b'0';
        *back_idx -= 2;
        if *back_idx >= array_idx {
            fill_remaining(res_p1, buf, array_idx, gen_idx, back_idx);
        }
    }
}
