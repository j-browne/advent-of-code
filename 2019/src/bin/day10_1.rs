use aoc_2019::asteroid_field::SpaceObject;
use ordered_float::OrderedFloat;
use std::{
    collections::HashSet,
    convert::TryFrom,
    io::{stdin, BufRead},
};

fn main() {
    println!("{}", day10_1(stdin().lock()));
}

fn day10_1(input: impl BufRead) -> usize {
    let asteroid_field = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.as_ref()
                .unwrap()
                .char_indices()
                .map(|(j, x)| {
                    (
                        (i32::try_from(i).unwrap(), i32::try_from(j).unwrap()),
                        SpaceObject::try_from(x).unwrap(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .filter_map(|(p, x)| (x == SpaceObject::Asteroid).then_some(p))
        .collect::<HashSet<_>>();

    asteroid_field
        .iter()
        .map(|(x0, y0)| {
            asteroid_field
                .iter()
                .filter_map(|(x1, y1)| {
                    (!(x0 == x1 && y0 == y1)).then_some(OrderedFloat(f64::atan2(
                        f64::from(x1 - x0),
                        f64::from(y1 - y0),
                    )))
                })
                .collect::<HashSet<_>>()
                .len()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    #[test]
    fn day10_1() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day10_1(BufReader::new(File::open("input/input_day10.txt").unwrap())),
            276
        );
    }
}
