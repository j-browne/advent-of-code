use aoc_2021::origami::Grid;

fn main() {
    println!("{}", run(include_str!("input/day_13.txt")));
}

fn run(input: &str) -> String {
    let mut grid = Grid::new(input);
    grid.fold();
    format!("{}", grid)
}

mod test {
    #[test]
    fn day_13_02_test() {
        assert_eq!(
            super::run(include_str!("input/day_13_test.txt")),
            "\
            #####\n\
            #...#\n\
            #...#\n\
            #...#\n\
            #####\n\
            .....\n\
            .....\n"
        );
    }

    #[test]
    fn day_13_02() {
        assert_eq!(
            super::run(include_str!("input/day_13.txt")),
            "\
            ###..#..#..##..#....###...##..###...##..\n\
            #..#.#..#.#..#.#....#..#.#..#.#..#.#..#.\n\
            #..#.####.#..#.#....#..#.#....#..#.#..#.\n\
            ###..#..#.####.#....###..#....###..####.\n\
            #.#..#..#.#..#.#....#.#..#..#.#.#..#..#.\n\
            #..#.#..#.#..#.####.#..#..##..#..#.#..#.\n"
        );
    }
}
