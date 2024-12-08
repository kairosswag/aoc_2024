use std::io::BufRead;

struct Equation {
    result: usize,
    operands: Vec<usize>,
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
}

pub fn run<R>(reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut equations = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.split_whitespace();
        let mut result = line.next().unwrap();
        let result = result[..result.len() - 1].parse::<usize>().unwrap();

        let operands = line.map(|val| val.parse::<usize>().unwrap()).collect();
        equations.push(Equation { result, operands });
    }

    let res_p1 = equations
        .iter()
        .filter(|eq| eq.may_be_valid())
        .map(|eq| eq.result)
        .sum();

    (res_p1, 5)
}
