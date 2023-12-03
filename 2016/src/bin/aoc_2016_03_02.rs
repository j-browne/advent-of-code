use std::str::FromStr;

fn main() {
    println!("{}", run(include_str!("input/day_03.txt")));
}

fn run(s: &str) -> u32 {
    let mut sum = 0;
    let mut vecs = [Vec::<u32>::new(), Vec::<u32>::new(), Vec::<u32>::new()];

    for (line_no, line) in s.trim().split('\n').enumerate() {
        let sides = line
            .trim()
            .split_whitespace()
            .map(u32::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        for i in 0..3 {
            vecs[i].push(sides[i]);
        }

        if line_no % 3 == 2 {
            for v in vecs.iter_mut() {
                v.sort_unstable();
                if v[0] + v[1] > v[2] {
                    sum += 1;
                }
                v.clear();
            }
        }
    }

    sum
}

mod test {
    #[test]
    fn day_03_02() {
        assert_eq!(super::run(include_str!("input/day_03.txt")), 1826);
    }
}
