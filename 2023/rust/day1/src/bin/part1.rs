use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut sum: u32 = 0;
    for line in reader.lines() {
        let mut first_digit: char = '0';
        let mut last_digit: char = '0';
        let mut is_first = true;

        for c in line?.chars() {
            if c.is_ascii_digit() {
                match is_first {
                    true => {
                        first_digit = c;
                        last_digit = c;
                        is_first = false;
                    }
                    false => {
                        last_digit = c;
                    }
                }
            }
        }

        let calibration_value: String = format!("{}{}", first_digit, last_digit);
        let calibration_value: u32 = calibration_value.parse().unwrap();
        sum += calibration_value;
    }

    println!("{}", sum);

    Ok(())
}
