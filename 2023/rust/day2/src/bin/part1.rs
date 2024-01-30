use core::panic;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
enum State {
    WaitingGameId,
    ReadingGameId,
    WaitingInteger,
    ReadingInteger,
    WaitingColor,
    ReadingColor,
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let max_reds = 12;
    let max_greens = 13;
    let max_blues = 14;

    let mut sum: u32 = 0;

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut max_game_reds: u32 = 0;
        let mut max_game_greens: u32 = 0;
        let mut max_game_blues: u32 = 0;
        let mut game_id: u32 = 0;

        let mut int_str = String::from("");
        let mut color_str = String::from("");
        let mut game_id_str = String::from("");

        let mut s = State::WaitingGameId;
        let mut it = line.chars();
        let mut cc_opt = it.next();
        let mut nc_opt = it.next();
        while let Some(cc) = cc_opt {
            match s {
                State::WaitingGameId => {
                    if let Some(nc) = nc_opt {
                        if nc.is_ascii_digit() {
                            s = State::ReadingGameId;
                        }
                    }
                }
                State::ReadingGameId => {
                    if cc.is_ascii_digit() {
                        game_id_str.push(cc);
                    }
                    if let Some(nc) = nc_opt {
                        if !nc.is_ascii_digit() {
                            game_id = game_id_str.parse().unwrap();
                            s = State::WaitingInteger;
                        }
                    }
                }
                State::WaitingInteger => {
                    if let Some(nc) = nc_opt {
                        if nc.is_ascii_digit() {
                            s = State::ReadingInteger;
                        }
                    }
                }
                State::ReadingInteger => {
                    if cc.is_ascii_digit() {
                        int_str.push(cc);
                    }
                    if let Some(nc) = nc_opt {
                        if !nc.is_ascii_digit() {
                            s = State::WaitingColor;
                        }
                    }
                }
                State::WaitingColor => {
                    if let Some(nc) = nc_opt {
                        if nc.is_ascii_alphabetic() {
                            s = State::ReadingColor;
                        }
                    }
                }
                State::ReadingColor => {
                    if cc.is_ascii_alphabetic() {
                        color_str.push(cc);
                    }

                    if nc_opt.is_none() || !nc_opt.unwrap().is_ascii_alphabetic() {
                        let i = int_str.parse().unwrap();

                        match color_str.as_str() {
                            "red" => {
                                if i > max_game_reds {
                                    max_game_reds = i;
                                }
                            }
                            "green" => {
                                if i > max_game_greens {
                                    max_game_greens = i;
                                }
                            }
                            "blue" => {
                                if i > max_game_blues {
                                    max_game_blues = i;
                                }
                            }
                            _ => {
                                panic!(
                                    "Found unexpected color at line {} in input file. Got '{}'.",
                                    line_number, color_str
                                );
                            }
                        };

                        int_str = String::from("");
                        color_str = String::from("");

                        s = State::WaitingInteger;
                    }
                }
            }

            cc_opt = nc_opt;
            nc_opt = it.next();
        }

        if max_game_reds <= max_reds && max_game_greens <= max_greens && max_game_blues <= max_blues
        {
            sum += game_id;
        }

        // println!("{}", line);
        // println!("sum:{}, id:{}, r:{}, g:{}, b:{}", sum, game_id, max_game_reds, max_game_greens, max_game_blues);
        // println!();
    }

    println!("{}", sum);

    Ok(())
}
