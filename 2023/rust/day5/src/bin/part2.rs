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

        // Sort the maps using the start of the qrc range
        maps.seed_to_soil.sort_by_key(|range| range.src.start);
        maps.soil_to_fertilizer.sort_by_key(|range| range.src.start);
        maps.fertilizer_to_water
            .sort_by_key(|range| range.src.start);
        maps.water_to_light.sort_by_key(|range| range.src.start);
        maps.light_to_temperature
            .sort_by_key(|range| range.src.start);
        maps.temperature_to_humidity
            .sort_by_key(|range| range.src.start);
        maps.humidity_to_location
            .sort_by_key(|range| range.src.start);

        maps.seed_to_soil = maps
            .fill_map("seed", "soil")
            .expect("Source or destination is invalid. Typo? Unexpected data?");
        maps.soil_to_fertilizer = maps
            .fill_map("soil", "fertilizer")
            .expect("Source or destination is invalid. Typo? Unexpected data?");
        maps.fertilizer_to_water = maps
            .fill_map("fertilizer", "water")
            .expect("Source or destination is invalid. Typo? Unexpected data?");
        maps.water_to_light = maps
            .fill_map("water", "light")
            .expect("Source or destination is invalid. Typo? Unexpected data?");
        maps.light_to_temperature = maps
            .fill_map("light", "temperature")
            .expect("Source or destination is invalid. Typo? Unexpected data?");
        maps.temperature_to_humidity = maps
            .fill_map("temperature", "humidity")
            .expect("Source or destination is invalid. Typo? Unexpected data?");
        maps.humidity_to_location = maps
            .fill_map("humidity", "location")
            .expect("Source or destination is invalid. Typo? Unexpected data?");

        Ok(maps)
    }

    fn fill_map(&self, src: &str, dest: &str) -> Option<Map> {
        let map = self.get_map(src, dest)?;
        let mut new_map: Vec<RangeProduct> = vec![];
        let first_src_bound = map[0].src.start;
        if first_src_bound != u64::MIN {
            new_map.push(RangeProduct {
                src: u64::MIN..first_src_bound,
                dest: u64::MIN..first_src_bound,
            });
        }
        for i in 0..(map.len() - 1) {
            let rp = &map[i];
            let next_rp = &map[i + 1];

            new_map.push(rp.clone());

            if next_rp.src.start != rp.src.end {
                new_map.push(RangeProduct {
                    src: rp.src.end..next_rp.src.start,
                    dest: rp.src.end..next_rp.src.start,
                });
            }
        }
        let last_rp = &map[map.len() - 1];
        new_map.push(last_rp.clone());
        let last_src_bound = last_rp.src.end;
        if last_src_bound != u64::MAX {
            new_map.push(RangeProduct {
                src: last_src_bound..u64::MAX,
                dest: last_src_bound..u64::MAX,
            });
        }

        Some(new_map)
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

    /// Rather tedious and ugly, but it works.
    fn src_range_to_dest_ranges(
        &self,
        src: &str,
        dest: &str,
        src_range: &Range<u64>,
    ) -> Option<Vec<Range<u64>>> {
        let mut dest_ranges: Vec<Range<u64>> = vec![];
        let map = self.get_map(src, dest)?;
        let mut map_rp_it = map.iter();
        let mut map_rp_opt = map_rp_it.next();

        let mut dest_range_start: u64;
        loop {
            match map_rp_opt {
                Some(rp) => {
                    if rp.src.contains(&src_range.start) {
                        let offset = src_range.start - rp.src.start;
                        dest_range_start = rp.dest.start + offset;
                        break;
                    } else {
                        map_rp_opt = map_rp_it.next();
                    }
                }
                None => panic!("Should have found the start of `src_range` among map src ranges."),
            }
        }

        loop {
            match map_rp_opt {
                Some(rp) => {
                    if rp.src.contains(&src_range.end) {
                        let offset = src_range.end - rp.src.start;
                        dest_ranges.push(dest_range_start..rp.dest.start + offset);
                        break;
                    } else {
                        dest_ranges.push(dest_range_start..rp.dest.end);
                        map_rp_opt = map_rp_it.next();
                        if let Some(rp) = map_rp_opt {
                            dest_range_start = rp.dest.start;
                        }
                    }
                }
                None => panic!("Should have found the end of `src_range` among map src ranges."),
            }
        }

        Some(dest_ranges)
    }

    fn seed_to_loc_ranges(&self, seed_ranges: &mut [Range<u64>]) -> Vec<Range<u64>> {
        seed_ranges.sort_by_key(|range| range.start);
        let loc_ranges: Vec<Range<u64>> = seed_ranges
            .iter()
            .flat_map(|seed_range| {
                self.src_range_to_dest_ranges("seed", "soil", seed_range)
                    .expect("Source or destination is invalid. Typo? Unexpected data?")
            })
            .flat_map(|ref soil_range| {
                self.src_range_to_dest_ranges("soil", "fertilizer", soil_range)
                    .expect("Source or destination is invalid. Typo? Unexpected data?")
            })
            .flat_map(|ref fertilizer_range| {
                self.src_range_to_dest_ranges("fertilizer", "water", fertilizer_range)
                    .expect("Source or destination is invalid. Typo? Unexpected data?")
            })
            .flat_map(|ref water_range| {
                self.src_range_to_dest_ranges("water", "light", water_range)
                    .expect("Source or destination is invalid. Typo? Unexpected data?")
            })
            .flat_map(|ref light_range| {
                self.src_range_to_dest_ranges("light", "temperature", light_range)
                    .expect("Source or destination is invalid. Typo? Unexpected data?")
            })
            .flat_map(|ref temperature_range| {
                self.src_range_to_dest_ranges("temperature", "humidity", temperature_range)
                    .expect("Source or destination is invalid. Typo? Unexpected data?")
            })
            .flat_map(|ref humidity_range| {
                self.src_range_to_dest_ranges("humidity", "location", humidity_range)
                    .expect("Source or destination is invalid. Typo? Unexpected data?")
            })
            .collect();

        loc_ranges
    }
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Find the seeds
    let first_line = lines.next().unwrap().unwrap();
    let seeds_cap = SEEDS_RE.captures(&first_line).unwrap();
    let seed_numbers: Vec<u64> = NUM_RE
        .captures_iter(&seeds_cap["seeds"])
        .map(|c| c.extract())
        .map(|(_, [n])| n.parse::<u64>().unwrap())
        .collect();
    let mut seed_ranges: Vec<Range<u64>> = vec![];
    let mut i: usize = 0;
    let mut seed_range: Range<u64> = Range { start: 0, end: 0 };
    while i < seed_numbers.len() {
        if i % 2 == 0 {
            seed_range.start = seed_numbers[i];
        } else {
            seed_range.end = seed_range.start + seed_numbers[i];
            seed_ranges.push(seed_range.clone());
        }
        i += 1;
    }

    // Build the maps
    let maps = Maps::new(lines).unwrap();

    // Find to locations corresponding to seeds
    let mut loc_ranges = maps.seed_to_loc_ranges(&mut seed_ranges);

    // Find the lowest location
    assert!(
        !loc_ranges.is_empty(),
        "Expected to have at least one location range."
    );
    loc_ranges.sort_by_key(|range| range.start);
    let res = loc_ranges[0].start;

    println!("{}", res);
    Ok(())
}
