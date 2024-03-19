use lazy_static::lazy_static;
use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    ops::Range,
};

// See https://stackoverflow.com/questions/35169259/how-to-make-a-compiled-regexp-a-global-variable
lazy_static! {
    static ref SEEDS_RE: Regex = Regex::new(r"seeds: (?<seeds>(\d| )+)").unwrap();
    static ref NUM_RE: Regex = Regex::new(r"(?<n>\d+)").unwrap();
    static ref MAP_RE: Regex = Regex::new(r"(?<src>\w+)-to-(?<dest>\w+) map:").unwrap();
}

#[derive(Debug, Clone)]
struct RangeProduct {
    src: Range<u64>,
    dest: Range<u64>,
}

type Map = Vec<RangeProduct>;

#[derive(Debug, Clone)]
struct Maps {
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl Maps {
    fn new(lines: Lines<BufReader<File>>) -> Result<Maps, Box<dyn Error>> {
        let mut maps = Maps {
            seed_to_soil: Vec::new(),
            soil_to_fertilizer: Vec::new(),
            fertilizer_to_water: Vec::new(),
            water_to_light: Vec::new(),
            light_to_temperature: Vec::new(),
            temperature_to_humidity: Vec::new(),
            humidity_to_location: Vec::new(),
        };

        let mut cur_map: &mut Map = &mut maps.seed_to_soil;

        for line in lines {
            let line = line?;
            if line.is_empty() {
                continue;
            }

            let first_char = line
                .chars()
                .next()
                .expect("Expected at least one character in the line.");
            if first_char.is_alphabetic() {
                // We are beginning a new map
                let caps = MAP_RE.captures(&line).unwrap();
                cur_map = maps
                    .get_mut_map(&caps["src"], &caps["dest"])
                    .expect("Source or destination is invalid. Typo? Unexpected data?");
                continue;
            }

            // At this point, line must be a range specification
            let ns: Vec<u64> = NUM_RE
                .captures_iter(&line)
                .map(|c| c.extract())
                .map(|(_, [n])| n.parse::<u64>().unwrap())
                .collect();

            if ns.len() != 3 {
                return Err("Expected 3 numbers in range.".into());
            }

            let dest_range_start = ns[0];
            let src_range_start = ns[1];
            let range_len = ns[2];
            cur_map.push(RangeProduct {
                src: Range {
                    start: src_range_start,
                    end: src_range_start + range_len,
                },
                dest: Range {
                    start: dest_range_start,
                    end: dest_range_start + range_len,
                },
            });
        }

        Ok(maps)
    }

    fn get_map(&self, src: &str, dest: &str) -> Option<&Map> {
        let field = format!("{}_to_{}", src, dest);
        match &field[..] {
            "seed_to_soil" => Some(&self.seed_to_soil),
            "soil_to_fertilizer" => Some(&self.soil_to_fertilizer),
            "fertilizer_to_water" => Some(&self.fertilizer_to_water),
            "water_to_light" => Some(&self.water_to_light),
            "light_to_temperature" => Some(&self.light_to_temperature),
            "temperature_to_humidity" => Some(&self.temperature_to_humidity),
            "humidity_to_location" => Some(&self.humidity_to_location),
            _ => None,
        }
    }

    fn get_mut_map(&mut self, src: &str, dest: &str) -> Option<&mut Map> {
        let field = format!("{}_to_{}", src, dest);
        match &field[..] {
            "seed_to_soil" => Some(&mut self.seed_to_soil),
            "soil_to_fertilizer" => Some(&mut self.soil_to_fertilizer),
            "fertilizer_to_water" => Some(&mut self.fertilizer_to_water),
            "water_to_light" => Some(&mut self.water_to_light),
            "light_to_temperature" => Some(&mut self.light_to_temperature),
            "temperature_to_humidity" => Some(&mut self.temperature_to_humidity),
            "humidity_to_location" => Some(&mut self.humidity_to_location),
            _ => None,
        }
    }

    fn get_dest_from_src(&self, src: &str, dest: &str, src_val: u64) -> Option<u64> {
        let map = self.get_map(src, dest)?;
        let mut dest_val = src_val;
        for range_prod in map {
            if range_prod.src.binary_search(src_val) {
                let offset = src_val - range_prod.src.start;
                dest_val = range_prod.dest.start + offset;
            }
        }

        Some(dest_val)
    }

    fn seed_to_loc(&self, seed: u64) -> u64 {
        let soil = self
            .get_dest_from_src("seed", "soil", seed)
            .expect("Source or destination is invalid. Typo? Unexpected data?");
        let fertilizer = self
            .get_dest_from_src("soil", "fertilizer", soil)
            .expect("Source or destination is invalid. Typo? Unexpected data?");
        let water = self
            .get_dest_from_src("fertilizer", "water", fertilizer)
            .expect("Source or destination is invalid. Typo? Unexpected data?");
        let light = self
            .get_dest_from_src("water", "light", water)
            .expect("Source or destination is invalid. Typo? Unexpected data?");
        let temperature = self
            .get_dest_from_src("light", "temperature", light)
            .expect("Source or destination is invalid. Typo? Unexpected data?");
        let humidity = self
            .get_dest_from_src("temperature", "humidity", temperature)
            .expect("Source or destination is invalid. Typo? Unexpected data?");
        let location = self
            .get_dest_from_src("humidity", "location", humidity)
            .expect("Source or destination is invalid. Typo? Unexpected data?");

        location
    }
}

trait RangeExt<U64> {
    fn binary_search(&self, n: u64) -> bool;
}

impl RangeExt<u64> for Range<u64> {
    fn binary_search(&self, n: u64) -> bool {
        let mut start = self.start;
        let mut end = self.end - 1;
        let mut mid: u64;
        while start <= end {
            mid = (start + end) / 2;

            match n.cmp(&mid) {
                std::cmp::Ordering::Equal => {
                    return true;
                }
                std::cmp::Ordering::Less => {
                    end = mid - 1;
                }
                std::cmp::Ordering::Greater => {
                    start = mid + 1;
                }
            }
        }

        false
    }
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Find the seeds
    let first_line = lines.next().unwrap().unwrap();
    let seeds_cap = SEEDS_RE.captures(&first_line).unwrap();
    let seeds: Vec<u64> = NUM_RE
        .captures_iter(&seeds_cap["seeds"])
        .map(|c| c.extract())
        .map(|(_, [n])| n.parse::<u64>().unwrap())
        .collect();

    // Build the maps
    let maps = Maps::new(lines).unwrap();

    // Find to locations corresponding to seeds
    let mut locations: Vec<u64> = vec![];
    for seed in seeds {
        locations.push(maps.seed_to_loc(seed));
    }

    // Find the lowest location
    let res = *locations
        .iter()
        .min()
        .expect("Expected `locations` to not be empty.");

    println!("{}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let range: Range<u64> = Range { start: 0, end: 10 };

        for n in 0..10 {
            assert!(range.binary_search(n));
        }
        assert!(!range.binary_search(11));
    }
}
