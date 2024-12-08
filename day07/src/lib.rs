use std::io::BufRead;

pub struct Equation {
    pub result: usize,
    pub operands: Vec<usize>,
}

impl Equation {
    fn may_be_valid(&self) -> bool {
        Self::may_provide_result(self.result, &self.operands)
    }

    fn may_provide_result(result: usize, remainder: &[usize]) -> bool {
        if remainder.len() == 1 {
            return result == remainder[0];
        }

        let next_number = remainder[remainder.len() - 1];
        let remaining = &remainder[..remainder.len() - 1];
        if result % next_number == 0 {
            if Self::may_provide_result(result / next_number, remaining) {
                return true;
            }
        }
        if result >= next_number {
            return Self::may_provide_result(result - next_number, remaining);
        }
        false
    }

    pub fn may_be_valid_with_concat(&self) -> bool {
        Self::may_provide_result_concat(self.result, &self.operands)
    }
    fn may_provide_result_concat(result: usize, remainder: &[usize]) -> bool {
        if remainder.len() == 1 {
            return result == remainder[0];
        }
        if remainder.len() == 0 {
            return true;
        }
        let next_number = remainder[remainder.len() - 1];
        let remaining = &remainder[..remainder.len() - 1];
        if result % next_number == 0 {
            if Self::may_provide_result_concat(result / next_number, remaining) {
                return true;
            }
        }
        if result >= next_number {
            if Self::may_provide_result_concat(result - next_number, remaining) {
                return true;
            }
        }
        let next_numbers_str = format!("{}", next_number);
        let result_str = format!("{result}");
        // println!("trying next_nubmers concat {next_numbers_str} against result {}", result);
        if next_numbers_str.len() > result_str.len() {
            return false;
        }
        let start_idx = result_str.len() - next_numbers_str.len();
        if result_str[start_idx..] == next_numbers_str {
            // println!("ogogo, testing {} gainst {:?} now", result_str[..start_idx].parse::<usize>().unwrap(), &remainder[..remainder.len() - 1]);
            return Self::may_provide_result_concat(result_str[..start_idx].parse().unwrap(), &remainder[..remainder.len() - 1]);
        }

        false
    }
}

#[test]
fn test_stuff() {
    // let eq = Equation { result: 156, operands: vec![15, 6] };
    // let valid = eq.may_be_valid_with_concat();
    // assert!(valid);

    let eq2 = Equation { result: 7290, operands: vec![6, 8, 6, 15] };
    let valid2 = eq2.may_be_valid_with_concat();
    assert_eq!(true, valid2);
}

pub fn run<R>(reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut equations = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.split_whitespace();
        let result = line.next().unwrap();
        let result = result[..result.len() - 1].parse::<usize>().unwrap();

        let operands = line.map(|val| val.parse::<usize>().unwrap()).collect();
        equations.push(Equation { result, operands });
    }


    let mut res_p1 = 0;
    let mut additional_p2 = 0;
    for eq in &equations {
        if eq.may_be_valid() {
            res_p1 += eq.result;
        } else if eq.may_be_valid_with_concat() {
            additional_p2 += eq.result;
        }
    }

    (res_p1, res_p1 + additional_p2)
}
