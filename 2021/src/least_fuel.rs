pub fn least_fuel<F>(mut nums: Vec<i32>, f: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    nums.sort_unstable();
    let mut prev = None;

    for i in nums[0]..*nums.last().unwrap() {
        let deviation = nums.iter().map(|x| f(*x, i)).sum();
        if let Some(prev) = prev {
            if deviation > prev {
                break;
            }
        }
        prev = Some(deviation);
    }

    prev.unwrap()
}
