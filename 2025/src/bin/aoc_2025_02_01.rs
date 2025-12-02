use aoc_2025::gift_shop_ids::GiftShopIds;

fn main() {
    println!("{}", run(include_str!("input/aoc_2025_02.txt")));
}

fn run(input: &str) -> u64 {
    GiftShopIds::new(input).sum_invalid()
}

mod test {
    #[test]
    fn aoc_2025_02_01_test() {
        assert_eq!(
            super::run(include_str!("input/aoc_2025_02_test.txt")),
            1227775554
        );
    }

    #[test]
    fn aoc_2025_02_01() {
        assert_eq!(
            super::run(include_str!("input/aoc_2025_02.txt")),
            31839939622
        );
    }
}
