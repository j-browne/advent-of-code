// TODO: use Vec2d?
use crate::Vec2d;

#[derive(Debug)]
pub struct DiagnosticReport {
    data: Vec2d<u32>,
}

impl DiagnosticReport {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let numbers = s.split('\n').filter(|x| !x.is_empty());
        let num_numbers = numbers.clone().count();
        let number_size = numbers.clone().next().unwrap().len();
        let dim = (num_numbers, number_size);
        let data = numbers
            .flat_map(|n| {
                n.chars().map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!(),
                })
            })
            .collect();
        let data = Vec2d::new(dim, data);

        Self { data }
    }

    #[must_use]
    pub fn gamma_rate(&self) -> u32 {
        let dim = self.data.dim();
        let most_common = (0..(dim.1)).rev().map(|i| self.most_common_at(i));
        bin_to_u32(most_common)
    }

    #[must_use]
    pub fn epsilon_rate(&self) -> u32 {
        let dim = self.data.dim();
        u32::pow(2, u32::try_from(dim.1).unwrap()) - 1 - self.gamma_rate()
    }

    #[must_use]
    pub fn oxygen_generator_rating(&self) -> u32 {
        let dim = self.data.dim();
        let mut candidates = (0..(dim.0)).collect::<Vec<_>>();
        for x in 0..(dim.1) {
            if candidates.len() == 1 {
                break;
            }

            let most_common = most_common(candidates.iter().map(|y| &self.data[(x, *y)]));
            candidates.retain(|y| self.data[(x, *y)] == most_common);
        }

        let y = candidates[0];
        bin_to_u32((0..(dim.1)).rev().map(|x| self.data[(x, y)]))
    }

    #[must_use]
    pub fn co2_scrubber_rating(&self) -> u32 {
        let dim = self.data.dim();
        let mut candidates = (0..(dim.0)).collect::<Vec<_>>();
        for x in 0..(dim.1) {
            if candidates.len() == 1 {
                break;
            }

            let least_common = least_common(candidates.iter().map(|y| &self.data[(x, *y)]));
            candidates.retain(|y| self.data[(x, *y)] == least_common);
        }

        let y = candidates[0];
        bin_to_u32((0..(dim.1)).rev().map(|x| self.data[(x, y)]))
    }

    #[must_use]
    pub fn most_common_at(&self, i: usize) -> u32 {
        let dim = self.data.dim();
        assert!(i < dim.1);
        most_common(self.data.iter().skip(i).step_by(dim.1))
    }

    #[must_use]
    pub fn least_common_at(&self, i: usize) -> u32 {
        1 - self.most_common_at(i)
    }
}

fn most_common<I, V>(it: I) -> u32
where
    I: Iterator<Item = V> + Clone,
    V: std::ops::Deref<Target = u32>,
{
    let n = u32::try_from(it.clone().count()).unwrap();
    it.map(|x| *x).sum::<u32>() * 2 / n
}

fn least_common<I, V>(it: I) -> u32
where
    I: Iterator<Item = V> + Clone,
    V: std::ops::Deref<Target = u32>,
{
    1 - most_common(it)
}

fn bin_to_u32(it: impl Iterator<Item = u32>) -> u32 {
    Iterator::zip(it, (0..).map(|x| u32::pow(2, x)))
        .map(|(a, b)| a * b)
        .sum()
}
