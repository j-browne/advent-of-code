use aoc_2024::warehouse_robot::Warehouse;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_15.txt")));
}

fn run(input: &str) -> usize {
    let mut warehouse = Warehouse::new(input);
    warehouse.run();
    warehouse.gps_sum()
}

mod test {
    #[test]
    fn aoc_2024_15_01_test_1() {
        assert_eq!(
            super::run(include_str!("input/aoc_2024_15_test_1.txt")),
            2028
        );
    }

    #[test]
    fn aoc_2024_15_01_test_2() {
        assert_eq!(
            super::run(include_str!("input/aoc_2024_15_test_2.txt")),
            10092
        );
    }

    #[test]
    fn aoc_2024_15_01() {
        assert_eq!(super::run(include_str!("input/aoc_2024_15.txt")), 1430439);
    }
}
