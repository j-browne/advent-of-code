use std::ops::Range;

struct Map(Vec<MapItem>);

impl Map {
    #[must_use]
    fn new(s: &str) -> Self {
        Self(
            s.lines()
                .skip(1)
                .map(|x| {
                    let mut it = x.split_whitespace();
                    let dest_start = it.next().unwrap().parse::<u64>().unwrap();
                    let src_start = it.next().unwrap().parse::<u64>().unwrap();
                    let len = it.next().unwrap().parse::<u64>().unwrap();
                    MapItem {
                        src_range: Range {
                            start: src_start,
                            end: src_start + len,
                        },
                        dest_range: Range {
                            start: dest_start,
                            end: dest_start + len,
                        },
                    }
                })
                .collect(),
        )
    }

    #[must_use]
    fn map_vec(&self, srcs: &[Range<u64>]) -> Vec<Range<u64>> {
        let mut dests = Vec::new();
        for src in srcs {
            let mut remaining = src.clone();

            'w: while remaining.start < remaining.end {
                let mut next_closest = None;

                for map_item in &self.0 {
                    // Keep track of the next closest range, in case we are not in a range and need
                    // this info after the loop
                    if next_closest.is_none() && remaining.contains(&map_item.src_range.start) {
                        next_closest = Some(map_item.src_range.start);
                    }
                    if let Some(x) = next_closest {
                        if remaining.contains(&map_item.src_range.start)
                            && map_item.src_range.start < x
                        {
                            next_closest = Some(map_item.src_range.start);
                        }
                    }

                    match (map_item.map(remaining.start), map_item.map(remaining.end)) {
                        (Some(start), Some(end)) => {
                            dests.push(Range { start, end });
                            break 'w;
                        }
                        (Some(start), None) => {
                            let src_end = map_item.src_range.end;
                            let dest_end = map_item.dest_range.end;
                            dests.push(Range {
                                start,
                                end: dest_end,
                            });
                            remaining.start = src_end;
                            continue 'w;
                        }
                        _ => {}
                    }
                }

                if let Some(x) = next_closest {
                    dests.push(Range {
                        start: remaining.start,
                        end: x,
                    });
                    remaining.start = x;
                    continue 'w;
                }
                dests.push(remaining);
                break 'w;
            }
        }
        dests
    }
}

struct MapItem {
    src_range: Range<u64>,
    dest_range: Range<u64>,
}
impl MapItem {
    #[must_use]
    fn map(&self, src: u64) -> Option<u64> {
        self.src_range
            .contains(&src)
            .then(|| src - self.src_range.start + self.dest_range.start)
    }
}

pub struct Almanac {
    seeds: Vec<Range<u64>>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl Almanac {
    #[must_use]
    pub fn from_singles(s: &str) -> Self {
        let mut it = s.split("\n\n");

        let seeds = {
            it.next()
                .unwrap()
                .strip_prefix("seeds: ")
                .unwrap()
                .split_whitespace()
                .map(|x| {
                    let start = x.parse().unwrap();
                    let end = start + 1;
                    Range { start, end }
                })
                .collect()
        };

        let seed_to_soil = Map::new(it.next().unwrap());
        let soil_to_fertilizer = Map::new(it.next().unwrap());
        let fertilizer_to_water = Map::new(it.next().unwrap());
        let water_to_light = Map::new(it.next().unwrap());
        let light_to_temperature = Map::new(it.next().unwrap());
        let temperature_to_humidity = Map::new(it.next().unwrap());
        let humidity_to_location = Map::new(it.next().unwrap());

        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    #[must_use]
    pub fn from_ranges(s: &str) -> Self {
        let mut it = s.split("\n\n");

        let seeds = {
            let mut seeds = Vec::new();
            let mut it = it
                .next()
                .unwrap()
                .strip_prefix("seeds: ")
                .unwrap()
                .split_whitespace();
            while let (Some(start), Some(end)) = (it.next(), it.next()) {
                let start = start.parse::<u64>().unwrap();
                let end = start + end.parse::<u64>().unwrap();
                seeds.push(Range { start, end });
            }
            seeds
        };

        let seed_to_soil = Map::new(it.next().unwrap());
        let soil_to_fertilizer = Map::new(it.next().unwrap());
        let fertilizer_to_water = Map::new(it.next().unwrap());
        let water_to_light = Map::new(it.next().unwrap());
        let light_to_temperature = Map::new(it.next().unwrap());
        let temperature_to_humidity = Map::new(it.next().unwrap());
        let humidity_to_location = Map::new(it.next().unwrap());

        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    #[must_use]
    pub fn locations(&self) -> Vec<Range<u64>> {
        let soils = self.seed_to_soil.map_vec(&self.seeds);
        let fertilizers = self.soil_to_fertilizer.map_vec(&soils);
        let waters = self.fertilizer_to_water.map_vec(&fertilizers);
        let lights = self.water_to_light.map_vec(&waters);
        let temperatures = self.light_to_temperature.map_vec(&lights);
        let humidities = self.temperature_to_humidity.map_vec(&temperatures);
        self.humidity_to_location.map_vec(&humidities)
    }
}
