fn main() {
    println!("{}", run(include_str!("input/aoc_2024_01.txt")));
}

fn run(input: &str) -> u32 {
    let mut nums1 = Vec::new();
    let mut nums2 = Vec::new();
    for l in input.lines() {
        let mut nums = l.split_whitespace();
        nums1.push(nums.next().unwrap().parse::<i32>().unwrap());
        nums2.push(nums.next().unwrap().parse::<i32>().unwrap());
    }

    nums1.sort_unstable();
    nums2.sort_unstable();

    nums1
        .into_iter()
        .zip(nums2.into_iter())
        .map(|(n1, n2)| (n2 - n1).abs() as u32)
        .sum()
}

mod test {
    #[test]
    fn aoc_2024_01_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2024_01_test.txt")), 11);
    }

    #[test]
    fn aoc_2024_01_01() {
        assert_eq!(super::run(include_str!("input/aoc_2024_01.txt")), 2086478);
    }
}
