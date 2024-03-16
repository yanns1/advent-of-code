use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

use regex::Regex;

fn main() -> io::Result<()> {
    let mut res = 0;
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let card_re = Regex::new(
        r"Card +(?<card_number>\d+): +(?<winning_numbers>(\d| )+) +\| +(?<my_numbers>(\d| )+)",
    )
    .unwrap();
    let number_re = Regex::new(r"(?<n>\d+)").unwrap();

    let mut card_counts: HashMap<u32, u32> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut winning_numbers: Vec<u32> = vec![];
        let mut my_numbers: Vec<u32> = vec![];
        let captures = card_re.captures(&line).unwrap();

        let card_number: u32 = captures["card_number"].parse().unwrap();

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

        card_counts
            .entry(card_number)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        let current_count = card_counts.get(&card_number).unwrap().clone();
        for offset in 1..n_winning_numbers + 1 {
            card_counts
                .entry(card_number + offset)
                .and_modify(|count| *count += current_count)
                .or_insert(current_count);
        }
    }

    for (_, count) in &card_counts {
        res += count;
    }

    println!("{}", res);

    return Ok(());
}
