use aoc_2021::bits_decoder::Packet;

fn main() {
    println!("{}", run(include_str!("input/day_16.txt")));
}

fn run(input: &str) -> u64 {
    let packet = Packet::from_hex_str(input);
    packet.version_sum()
}

mod test {
    #[test]
    fn day_16_01_test_01() {
        assert_eq!(super::run(include_str!("input/day_16_01_test_01.txt")), 16);
    }

    #[test]
    fn day_16_01_test_02() {
        assert_eq!(super::run(include_str!("input/day_16_01_test_02.txt")), 12);
    }

    #[test]
    fn day_16_01_test_03() {
        assert_eq!(super::run(include_str!("input/day_16_01_test_03.txt")), 23);
    }

    #[test]
    fn day_16_01_test_04() {
        assert_eq!(super::run(include_str!("input/day_16_01_test_04.txt")), 31);
    }

    #[test]
    fn day_16_01() {
        assert_eq!(super::run(include_str!("input/day_16.txt")), 936);
    }
}
