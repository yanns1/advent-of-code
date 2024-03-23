use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

lazy_static! {
    static ref TIME_RE: Regex = Regex::new(r"Time: *(?<times>(\d| )+)").unwrap();
    static ref DISTANCE_RE: Regex = Regex::new(r"Distance: *(?<distances>(\d| )+)").unwrap();
    static ref NUM_RE: Regex = Regex::new(r"(?<n>\d+)").unwrap();
}

type Time = u64;
type Distance = u64;
type Speed = u64;

#[derive(Debug)]
struct Race {
    time: Time,                // ms
    record_distance: Distance, // mm
}

impl Race {
    fn new(time: Time, record_distance: Distance) -> Race {
        Race {
            time,
            record_distance,
        }
    }
}

#[derive(Debug)]
struct ToyBoat {
    speed_gained_by_ms_hold: Speed, // mm/ms
}

impl ToyBoat {
    fn new() -> ToyBoat {
        ToyBoat {
            speed_gained_by_ms_hold: 1,
        }
    }

    fn run_race(&self, race: &Race, hold: Time) -> Distance {
        let speed = hold * self.speed_gained_by_ms_hold;
        let remaining_time = race.time - hold;
        speed * remaining_time
    }
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Parse the race
    let first_line = lines.next().unwrap().unwrap();
    let time_caps = TIME_RE.captures(&first_line).unwrap();
    let mut time = String::new();
    for (_, [time_str]) in NUM_RE
        .captures_iter(&time_caps["times"])
        .map(|c| c.extract())
    {
        time.push_str(time_str);
    }
    let time: u64 = time.parse().unwrap();

    let second_line = lines.next().unwrap().unwrap();
    let distance_caps = DISTANCE_RE.captures(&second_line).unwrap();
    let mut distance = String::new();
    for (_, [distance_str]) in NUM_RE
        .captures_iter(&distance_caps["distances"])
        .map(|c| c.extract())
    {
        distance.push_str(distance_str);
    }
    let distance: u64 = distance.parse().unwrap();

    let race = Race::new(time, distance);

    // Find the number of ways to do better than the records
    let boat = ToyBoat::new();
    let mut n_ways_to_do_worse: u64 = 0;
    let mut hold: u64 = 0;
    while hold <= race.time && boat.run_race(&race, hold) <= race.record_distance {
        n_ways_to_do_worse += 1;
        hold += 1;
    }

    let mut n_ways_to_do_better: u64 = race.time / 2;
    if race.time % 2 == 1 {
        n_ways_to_do_better += 1;
    }
    n_ways_to_do_better = (n_ways_to_do_better - n_ways_to_do_worse) * 2;

    println!("{}", n_ways_to_do_better);

    Ok(())
}
