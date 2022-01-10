use aoc_2021::monad::Monad;

fn main() {
    println!("{}", run(include_str!("input/day_24.txt")));
}

fn run(input: &str) -> u32 {
    let _monad = Monad::new(input);
    todo!()
}

mod test {
    #[test]
    fn day_24_01() {
        assert_eq!(super::run(include_str!("input/day_24.txt")), 0);
    }
}
