// Allow these casts because there is no int/float TryFrom
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]

use std::iter;

pub struct Race {
    time: u64,
    record: u64,
}

pub struct Races {
    races: Vec<Race>,
}

impl Races {
    #[must_use]
    pub fn with_spaces(s: &str) -> Self {
        let mut lines = s.lines();
        let times = lines
            .next()
            .unwrap()
            .strip_prefix("Time:")
            .unwrap()
            .split_whitespace();
        let records = lines
            .next()
            .unwrap()
            .strip_prefix("Distance:")
            .unwrap()
            .split_whitespace();

        let races = iter::zip(times, records)
            .map(|(time, record)| {
                let time = time.parse().unwrap();
                let record = record.parse().unwrap();
                Race { time, record }
            })
            .collect();
        Races { races }
    }

    #[must_use]
    pub fn without_spaces(s: &str) -> Self {
        let mut lines = s.lines();
        let time = lines
            .next()
            .unwrap()
            .strip_prefix("Time:")
            .unwrap()
            .replace(' ', "")
            .parse()
            .unwrap();
        let record = lines
            .next()
            .unwrap()
            .strip_prefix("Distance:")
            .unwrap()
            .replace(' ', "")
            .parse()
            .unwrap();

        Races {
            races: vec![Race { time, record }],
        }
    }

    #[must_use]
    pub fn num_ways_to_win(&self) -> Vec<u64> {
        self.races
            .iter()
            .map(|race| {
                let time = race.time as f64;
                let record = race.record as f64;

                let discriminant = time.powi(2) - 4.0 * record;
                assert!(discriminant > 0.0);
                let upper = (time + f64::sqrt(discriminant)) / 2.0;
                let lower = (time - f64::sqrt(discriminant)) / 2.0;

                // This trickery handles the edge cases where upper or lower are exactly an
                // integer. Ties should _not_ be counted
                (upper - 1.0).ceil() as u64 - (lower + 1.0).floor() as u64 + 1
            })
            .collect()
    }
}
