use aoc_2021::dumbo_octopus::Grid;

fn main() {
    println!("{}", run(include_str!("input/day_11.txt")));
}

fn run(input: &str) -> u32 {
    let mut grid = Grid::new(input);
    let mut i = 0;
    let mut last_flashes = 0;
    loop {
        i += 1;
        grid.step();
        if grid.flashes() - last_flashes == 100 {
            break;
        }
        last_flashes = grid.flashes();
    }
    i
}

mod test {
    #[test]
    fn day_11_02_test() {
        assert_eq!(super::run(include_str!("input/day_11_test.txt")), 195);
    }

    #[test]
    fn day_11_02() {
        assert_eq!(super::run(include_str!("input/day_11.txt")), 403);
    }
}
