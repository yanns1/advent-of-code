use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    iter::zip,
};

lazy_static! {
    static ref TIME_RE: Regex = Regex::new(r"Time: *(?<times>(\d| )+)").unwrap();
    static ref DISTANCE_RE: Regex = Regex::new(r"Distance: *(?<distances>(\d| )+)").unwrap();
    static ref NUM_RE: Regex = Regex::new(r"(?<n>\d+)").unwrap();
}

type Time = u32;
type Distance = u32;
type Speed = u32;

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

    // Parse the races
    let first_line = lines.next().unwrap().unwrap();
    let time_caps = TIME_RE.captures(&first_line).unwrap();
    let times: Vec<u32> = NUM_RE
        .captures_iter(&time_caps["times"])
        .map(|c| c.extract())
        .map(|(_, [n])| n.parse::<u32>().unwrap())
        .collect();

    let second_line = lines.next().unwrap().unwrap();
    let distance_caps = DISTANCE_RE.captures(&second_line).unwrap();
    let distances: Vec<u32> = NUM_RE
        .captures_iter(&distance_caps["distances"])
        .map(|c| c.extract())
        .map(|(_, [n])| n.parse::<u32>().unwrap())
        .collect();

    assert_eq!(
        times.len(),
        distances.len(),
        "Expected to have one time per distance, and vice versa."
    );
    let races: Vec<Race> = zip(&times, &distances)
        .map(|(&t, &d)| Race::new(t, d))
        .collect();

    // Find all the ways to do better than the records
    let mut n_ways_to_beat_record_per_race: Vec<u32> = vec![];
    let boat = ToyBoat::new();
    for race in races {
        let mut n_ways: u32 = 0;
        for hold in 1..race.time {
            let d = boat.run_race(&race, hold);
            if d > race.record_distance {
                n_ways += 1;
            }
        }
        n_ways_to_beat_record_per_race.push(n_ways);
    }

    // Aggregate to obtain the result
    let res: u32 = n_ways_to_beat_record_per_race.iter().product();

    println!("{}", res);

    Ok(())
}

