use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
    rank: Rank,
}

impl Hand {
    #[must_use]
    pub fn without_jokers(s: &str) -> Self {
        let cards = s
            .chars()
            .map(|c| Card::new(c, false))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let rank = calc_rank(cards);
        Self { cards, rank }
    }

    #[must_use]
    pub fn with_jokers(s: &str) -> Self {
        let cards = s
            .chars()
            .map(|c| Card::new(c, true))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let rank = calc_rank(cards);
        Self { cards, rank }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        Rank::cmp(&self.rank, &other.rank)
            .then_with(|| Card::cmp(&self.cards[0], &other.cards[0]))
            .then_with(|| Card::cmp(&self.cards[1], &other.cards[1]))
            .then_with(|| Card::cmp(&self.cards[2], &other.cards[2]))
            .then_with(|| Card::cmp(&self.cards[3], &other.cards[3]))
            .then_with(|| Card::cmp(&self.cards[4], &other.cards[4]))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn calc_rank(cards: [Card; 5]) -> Rank {
    let mut map = HashMap::new();
    for card in cards {
        *map.entry(card).or_insert(0) += 1;
    }

    let num_jokers = map.remove(&Card::Joker).unwrap_or(0);

    let mut counts = map.values().collect::<Vec<_>>();
    counts.sort_by(|x1, x2| x2.cmp(x1));

    match (**counts.first().unwrap_or(&&0) + num_jokers, counts.get(1)) {
        (5, _) => Rank::FiveOfAKind,
        (4, _) => Rank::FourOfAKind,
        (3, Some(2)) => Rank::FullHouse,
        (3, _) => Rank::ThreeOfAKind,
        (2, Some(2)) => Rank::TwoPair,
        (2, _) => Rank::Pair,
        _ => Rank::HighCard,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn new(value: char, with_jokers: bool) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => {
                if with_jokers {
                    Card::Joker
                } else {
                    Card::Jack
                }
            }
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("unknown card"),
        }
    }
}
