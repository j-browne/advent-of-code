fn main() {
    println!("{}", run(include_str!("input/aoc_2024_01.txt")));
}

fn run(input: &str) -> u32 {
    let mut nums1 = Vec::new();
    let mut nums2 = Vec::new();
    for l in input.lines() {
        let mut nums = l.split_whitespace();
        nums1.push(nums.next().unwrap().parse::<u32>().unwrap());
        nums2.push(nums.next().unwrap().parse::<u32>().unwrap());
    }

    nums2.sort_unstable();

    nums1
        .into_iter()
        .map(|n1| n1 * u32::try_from(nums2.iter().filter(|n2| n1 == **n2).count()).unwrap())
        .sum()
}

mod test {
    #[test]
    fn aoc_2024_01_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2024_01_test.txt")), 31);
    }

    #[test]
    fn aoc_2024_01_02() {
        assert_eq!(super::run(include_str!("input/aoc_2024_01.txt")), 24941624);
    }
}
