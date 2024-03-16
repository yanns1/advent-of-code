use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
enum State {
    WaitingInt,
    ReadingInt,
}

#[derive(Debug)]
struct Number {
    number: u32,
    line_number: usize,
    start_col: usize,
    end_col: usize,
    prev_line: Option<(usize, String)>,
    line: (usize, String),
    next_line: Option<(usize, String)>,
}

fn is_special_char(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}

fn is_part_number(n: &Number) -> bool {
    // Check if there is a special char in previous line
    if let Some((_, ref prev_line)) = n.prev_line {
        for c in (&(*prev_line)[n.start_col.checked_sub(1).unwrap_or(0)
            ..n.end_col.checked_add(1).unwrap_or(n.end_col) + 1])
            .chars()
        {
            if is_special_char(c) {
                return true;
            }
        }
    }

    // Check if there is a special char on the sides
    let (_, ref line) = n.line;
    let left_idx = n.start_col.checked_sub(1).unwrap_or(0);
    for c in (&line[left_idx..left_idx + 1]).chars() {
        if is_special_char(c) {
            return true;
        }
    }
    let right_idx = n.end_col.checked_add(1).unwrap_or(n.end_col);
    for c in (&line[right_idx..right_idx + 1]).chars() {
        if is_special_char(c) {
            return true;
        }
    }

    // Check if there is a special char in next line
    if let Some((_, ref next_line)) = n.next_line {
        for c in (&(*next_line)[n.start_col.checked_sub(1).unwrap_or(0)
            ..n.end_col.checked_add(1).unwrap_or(n.end_col) + 1])
            .chars()
        {
            if is_special_char(c) {
                return true;
            }
        }
    }

    return false;
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut res: u32 = 0;

    let mut lines_it = reader.lines().enumerate();
    let mut prev_line_opt: Option<(usize, Result<String, std::io::Error>)> = None;
    let mut line_opt = lines_it.next();
    let mut next_line_opt = lines_it.next();

    let mut n = Number {
        number: 0,
        line_number: 0,
        start_col: 0,
        end_col: 0,
        prev_line: None,
        line: (0, String::from("")),
        next_line: None,
    };

    while let Some((line_number, ref line)) = line_opt {
        let line = line.as_ref().unwrap();

        let prev_line_opt_bis = match prev_line_opt {
            Some((_0, _1)) => Some((_0, _1.unwrap())),
            None => None,
        };
        let next_line_opt_bis = match next_line_opt {
            Some((_0, ref _1)) => Some((_0, _1.as_ref().unwrap().clone())),
            None => None,
        };

        n.line_number = line_number;
        n.prev_line = prev_line_opt_bis;
        n.line = (line_number, line.clone());
        n.next_line = next_line_opt_bis;

        let mut it = line.chars().enumerate();
        let mut cc_opt = it.next();
        let mut nc_opt = it.next();
        let mut n_str = String::from("");
        let mut s = State::WaitingInt;
        if let Some((_, cc)) = cc_opt {
            if cc.is_ascii_digit() {
                s = State::ReadingInt;
            };
        }
        while let Some((cc_col, cc)) = cc_opt {
            match s {
                State::WaitingInt => {
                    if let Some((nc_col, nc)) = nc_opt {
                        if nc.is_ascii_digit() {
                            n.start_col = nc_col;
                            s = State::ReadingInt;
                        }
                    };
                }
                State::ReadingInt => {
                    n_str.push(cc);

                    if let Some((_, nc)) = nc_opt {
                        if !nc.is_ascii_digit() {
                            n.number = n_str.parse().unwrap();
                            n_str = String::from("");
                            n.end_col = cc_col;

                            if is_part_number(&n) {
                                res += n.number;
                            }

                            n = Number {
                                number: 0,
                                start_col: 0,
                                end_col: 0,
                                ..n
                            };

                            s = State::WaitingInt;
                        }
                    }
                }
            }

            cc_opt = nc_opt;
            nc_opt = it.next();
        }

        if n_str.chars().count() > 0 {
            n.number = n_str.parse().unwrap();
            n.end_col = n.line.1.chars().count() - 2;
            if is_part_number(&n) {
                res += n.number;
            }
        }

        prev_line_opt = line_opt;
        line_opt = next_line_opt;
        next_line_opt = lines_it.next();
    }

    println!("{}", res);

    Ok(())
}
