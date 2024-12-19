use std::io::BufRead;

pub fn run<R>(reader: R) -> (usize, usize)
where
    R: BufRead,
{

    let mut regex_groups: Vec<String> = Vec::new();
    let mut lines = reader.lines();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }
        for expr in line.split(",").map(|s| s.trim()) {
            regex_groups.push(expr.to_string());
        }
    }
    let groups = format!("^(({}))*$", regex_groups.join(")|("));
    let regex = regex::Regex::new(&groups);
    let regex = regex.unwrap();
    let mut count_p1 = 0;
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap().unwrap();
        if line.is_empty() {
            break;
        }

        if regex.is_match(&line) {
            count_p1 += 1;
        }

    }

    (count_p1, 5)
    
}



