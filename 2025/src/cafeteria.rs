use std::mem::swap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct IngredientRange(u64, u64);

impl IngredientRange {
    #[must_use]
    fn new(s: &str) -> Self {
        let (first, second) = s.split_once('-').unwrap();
        Self(first.parse().unwrap(), second.parse().unwrap())
    }

    #[must_use]
    fn contains(&self, ingredient: u64) -> bool {
        ingredient >= self.0 && ingredient <= self.1
    }

    #[must_use]
    pub fn count(&self) -> u64 {
        self.1 - self.0 + 1
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inventory {
    fresh_ranges: Vec<IngredientRange>,
    ingredients: Vec<u64>,
}

impl Inventory {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let (fresh_ranges, ingredients) = s.split_once("\n\n").unwrap();
        let mut fresh_ranges = fresh_ranges.lines().map(IngredientRange::new).collect();
        condense_ranges(&mut fresh_ranges);
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
            .filter(|i| self.fresh_ranges.iter().any(|r| r.contains(**i)))
            .count()
    }

    #[must_use]
    pub fn range_count(&self) -> u64 {
        self.fresh_ranges.iter().map(IngredientRange::count).sum()
    }
}

fn condense_ranges(ranges: &mut Vec<IngredientRange>) {
    ranges.sort_by_key(|x| x.0);
    let mut temp_ranges = Vec::<IngredientRange>::new();

    let mut changed = true;
    while changed {
        changed = false;
        for IngredientRange(first, second) in &mut *ranges {
            let mut added = false;

            for range in &mut temp_ranges {
                if range.contains(*first) {
                    if *second > range.1 {
                        range.1 = *second;
                        changed = true;
                    }
                    added = true;
                    break;
                }
                if range.contains(*second) {
                    if *first < range.0 {
                        range.0 = *first;
                        changed = true;
                    }
                    added = true;
                    break;
                }
            }

            if !added {
                temp_ranges.push(IngredientRange(*first, *second));
            }
        }
        swap(ranges, &mut temp_ranges);
        temp_ranges.clear();
    }
}
