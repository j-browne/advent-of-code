use std::collections::HashMap;

pub struct CubeGame {
    pull_maxima: HashMap<String, u32>,
}

impl CubeGame {
    pub fn new(s: &str) -> Self {
        let mut pull_maxima = HashMap::new();

        for pull in s.split("; ") {
            for num_color in pull.split(", ") {
                let mut num_color = num_color.split_whitespace();
                let num = num_color.next().unwrap().parse::<u32>().unwrap();
                let color = num_color.next().unwrap();

                let max = pull_maxima.entry(color.to_string()).or_insert(0);
                if num > *max {
                    *max = num;
                }
            }
        }

        Self { pull_maxima }
    }

    pub fn is_valid_for(&self, bag_maxima: &HashMap<String, u32>) -> bool {
        for (color, max) in &self.pull_maxima {
            if max > bag_maxima.get(color).unwrap_or(&0) {
                return false;
            }
        }
        true
    }

    pub fn power(&self) -> u32 {
        self.pull_maxima.iter().map(|(_, n)| n).product::<u32>()
    }
}
