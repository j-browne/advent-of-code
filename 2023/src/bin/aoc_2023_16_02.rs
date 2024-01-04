use aoc_2023::{array_2d::Indices, dir::Dir4, mirror_map::MirrorMap};
use std::iter::empty;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_16.txt")));
}

fn run(input: &str) -> usize {
    let map = MirrorMap::new(input);
    empty()
        .chain(
            map.dims()
                .rows
                .map(|row| map.num_energized(Indices { row, col: 0 }, Dir4::Right)),
        )
        .chain(map.dims().rows.map(|row| {
            map.num_energized(
                Indices {
                    col: map.dims().cols.end - 1,
                    row,
                },
                Dir4::Left,
            )
        }))
        .chain(
            map.dims()
                .cols
                .map(|col| map.num_energized(Indices { col, row: 0 }, Dir4::Down)),
        )
        .chain(map.dims().cols.map(|col| {
            map.num_energized(
                Indices {
                    col,
                    row: map.dims().rows.end - 1,
                },
                Dir4::Up,
            )
        }))
        .max()
        .unwrap()
}

mod test {
    #[test]
    fn aoc_2023_16_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_16_test.txt")), 51);
    }

    #[test]
    fn aoc_2023_16_02() {
        assert_eq!(super::run(include_str!("input/aoc_2023_16.txt")), 8061);
    }
}
