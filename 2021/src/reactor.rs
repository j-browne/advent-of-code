pub struct Reactor {
    cores: Vec<bool>,
}

impl Reactor {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut cores = vec![false; 101 * 101 * 101];
        for line in s.trim().split('\n') {
            let mut it = line.split(' ');
            let state = match it.next().unwrap() {
                "on" => true,
                "off" => false,
                x => panic!("unknown state `{}`", x),
            };

            let mut it = it.next().unwrap().split(',');
            let x = parse_range(it.next().unwrap(), "x");
            let y = parse_range(it.next().unwrap(), "y");
            let z = parse_range(it.next().unwrap(), "z");

            for z in (z.0)..=(z.1) {
                for y in (y.0)..=(y.1) {
                    for x in (x.0)..=(x.1) {
                        if let Some(idx) = idx(x, y, z) {
                            cores[idx] = state;
                        }
                    }
                }
            }
        }

        Self { cores }
    }

    #[must_use]
    pub fn num_on(&self) -> usize {
        self.cores.iter().filter(|x| **x).count()
    }
}

fn idx(x: isize, y: isize, z: isize) -> Option<usize> {
    if x >= -50 && x <= 50 && y >= -50 && y <= 50 && z >= -50 && z <= 50 {
        let x: usize = (x + 50).try_into().unwrap();
        let y: usize = (y + 50).try_into().unwrap();
        let z: usize = (z + 50).try_into().unwrap();
        Some(101 * 101 * z + 101 * y + x)
    } else {
        None
    }
}

fn parse_range(s: &str, dim: &'static str) -> (isize, isize) {
    let mut it = s.split('=');
    assert_eq!(it.next().unwrap(), dim);
    let mut it = it.next().unwrap().split("..");
    (
        it.next().unwrap().parse::<isize>().unwrap().clamp(-51, 51),
        it.next().unwrap().parse::<isize>().unwrap().clamp(-51, 51),
    )
}
