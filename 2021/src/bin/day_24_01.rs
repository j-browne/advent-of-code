use aoc_2021::monad::Monad;

fn main() {
    println!("{}", run(include_str!("input/day_24.txt")));
}

fn run(input: &str) -> u64 {
    let monad = Monad::new(input);
    monad.largest_valid_model_number()
}

mod test {
    #[test]
    fn day_24_01() {
        assert_eq!(super::run(include_str!("input/day_24.txt")), 0);
    }
}
