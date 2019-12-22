#![feature(bool_to_option)]
use aoc_2019::asteroid_field::SpaceObject;
use ordered_float::OrderedFloat;
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
    f64::consts::PI,
    io::{stdin, BufRead},
};

fn main() {
    println!("{}", day10_2(stdin().lock()));
}

fn day10_2(input: impl BufRead) -> i32 {
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

    let pos = asteroid_field
        .iter()
        .max_by_key(|(x0, y0)| {
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
        .unwrap();

    let angles = asteroid_field.iter().fold(
        HashMap::<OrderedFloat<f64>, Vec<(i32, i32)>>::new(),
        |mut map, (x, y)| {
            if !(pos.0 == *x && pos.1 == *y) {
                let mut angle = f64::atan2(f64::from(*y - pos.1), f64::from(pos.0 - *x));
                if angle < 0.0 {
                    angle += 2.0 * PI;
                }
                let angle = OrderedFloat(angle);
                map.entry(angle).or_insert_with(Vec::new).push((*x, *y));
            }
            map
        },
    );

    let mut angles = angles.into_iter().collect::<Vec<_>>();

    angles.sort_unstable_by_key(|x| x.0);
    for (_, vec) in &mut angles {
        vec.sort_unstable_by_key(|(x, y)| -((pos.0 - x) * (pos.0 - x) + (pos.1 - y) * (pos.1 - y)));
    }

    let mut last = None;
    let mut countdown = 200;
    while countdown > 0 {
        for (_angle, vec) in &mut angles {
            last = vec.pop();
            if last.is_some() {
                countdown -= 1;
            }
            if countdown == 0 {
                break;
            }
        }
    }

    let last = last.unwrap();
    100 * last.1 + last.0
}

#[cfg(test)]
mod test {
    #[test]
    fn day10_2() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day10_2(BufReader::new(File::open("input/input_day10.txt").unwrap())),
            1321
        );
    }
}
