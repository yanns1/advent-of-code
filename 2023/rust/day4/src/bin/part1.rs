use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use regex::Regex;

fn main() -> io::Result<()> {
    let mut res = 0;
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let card_re =
        Regex::new(r"Card +\d+: +(?<winning_numbers>(\d| )+) +\| +(?<my_numbers>(\d| )+)").unwrap();
    let number_re = Regex::new(r"(?<n>\d+)").unwrap();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut winning_numbers: Vec<u32> = vec![];
        let mut my_numbers: Vec<u32> = vec![];
        let captures = card_re.captures(&line).unwrap();

        for (_, [n]) in number_re
            .captures_iter(&captures["winning_numbers"])
            .map(|c| c.extract())
        {
            winning_numbers.push(n.parse().unwrap());
        }
        for (_, [n]) in number_re
            .captures_iter(&captures["my_numbers"])
            .map(|c| c.extract())
        {
            my_numbers.push(n.parse().unwrap());
        }

        let n_winning_numbers = my_numbers.iter().fold(0, |acc, n| {
            if winning_numbers.contains(n) {
                acc + 1
            } else {
                acc
            }
        });

        if n_winning_numbers > 0 {
            res += 2_u32.pow(n_winning_numbers - 1);
        }
    }

    println!("{}", res);

    return Ok(());
}
