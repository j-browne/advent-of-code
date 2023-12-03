use aoc_2021::bits_decoder::Packet;

fn main() {
    println!("{}", run(include_str!("input/day_16.txt")));
}

fn run(input: &str) -> u64 {
    let packet = Packet::from_hex_str(input);
    packet.value()
}

mod test {
    #[test]
    fn day_16_02_test_01() {
        assert_eq!(super::run(include_str!("input/day_16_02_test_01.txt")), 3);
    }

    #[test]
    fn day_16_02_test_02() {
        assert_eq!(super::run(include_str!("input/day_16_02_test_02.txt")), 54);
    }

    #[test]
    fn day_16_02_test_03() {
        assert_eq!(super::run(include_str!("input/day_16_02_test_03.txt")), 7);
    }

    #[test]
    fn day_16_02_test_04() {
        assert_eq!(super::run(include_str!("input/day_16_02_test_04.txt")), 9);
    }

    #[test]
    fn day_16_02_test_05() {
        assert_eq!(super::run(include_str!("input/day_16_02_test_05.txt")), 1);
    }

    #[test]
    fn day_16_02_test_06() {
        assert_eq!(super::run(include_str!("input/day_16_02_test_06.txt")), 0);
    }

    #[test]
    fn day_16_02_test_07() {
        assert_eq!(super::run(include_str!("input/day_16_02_test_07.txt")), 0);
    }

    #[test]
    fn day_16_02_test_08() {
        assert_eq!(super::run(include_str!("input/day_16_02_test_08.txt")), 1);
    }

    #[test]
    fn day_16_02() {
        assert_eq!(super::run(include_str!("input/day_16.txt")), 6802496672062);
    }
}
