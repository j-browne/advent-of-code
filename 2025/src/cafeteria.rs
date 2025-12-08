use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq, Eq)]
struct IngredientRanges(Vec<RangeInclusive<u64>>);

impl IngredientRanges {
    #[must_use]
    fn new(s: &str) -> Self {
        let ranges = s
            .lines()
            .map(|l| {
                let (first, second) = l.split_once('-').unwrap();
                first.parse().unwrap()..=second.parse().unwrap()
            })
            .collect();
        Self(ranges)
    }

    #[must_use]
    fn contains(&self, ingredient: u64) -> bool {
        self.0.iter().any(|r| r.contains(&ingredient))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inventory {
    fresh_ranges: IngredientRanges,
    ingredients: Vec<u64>,
}

impl Inventory {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let (fresh_ranges, ingredients) = s.split_once("\n\n").unwrap();
        let fresh_ranges = IngredientRanges::new(fresh_ranges);
        let ingredients = ingredients.lines().map(|s| s.parse().unwrap()).collect();

        Self {
            fresh_ranges,
            ingredients,
        }
    }

    #[must_use]
    pub fn num_fresh(&self) -> usize {
        self.ingredients
            .iter()
            .filter(|i| self.fresh_ranges.contains(**i))
            .count()
    }
}
