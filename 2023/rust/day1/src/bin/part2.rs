use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut sum: u32 = 0;

    let hm: HashMap<String, char> = HashMap::from([
        (String::from("one"), '1'),
        (String::from("two"), '2'),
        (String::from("three"), '3'),
        (String::from("four"), '4'),
        (String::from("five"), '5'),
        (String::from("six"), '6'),
        (String::from("seven"), '7'),
        (String::from("eight"), '8'),
        (String::from("nine"), '9'),
    ]);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut only_digits = String::from("");

        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                only_digits.push(c);
            } else {
                for key in hm.keys() {
                    if line[i..].starts_with(key) {
                        only_digits.push(*hm.get(key).unwrap());
                    }
                }
            }
        }

        let calibration_value: String = format!(
            "{}{}",
            only_digits.chars().next().unwrap(),
            only_digits.chars().last().unwrap()
        );
        let calibration_value: u32 = calibration_value.parse().unwrap();
        sum += calibration_value;
    }

    println!("{}", sum);

    Ok(())
}
