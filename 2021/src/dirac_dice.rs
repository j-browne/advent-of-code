use std::collections::HashMap;

#[derive(Debug)]
pub struct DeterministicGame {
    scores: Vec<u32>,
    positions: Vec<u32>,
    num_rolls: u32,
}

impl DeterministicGame {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let positions = s
            .trim()
            .split('\n')
            .map(|s| s.split(": ").nth(1).unwrap().parse().unwrap())
            .collect::<Vec<_>>();

        let scores = vec![0; positions.len()];

        Self {
            scores,
            positions,
            num_rolls: 0,
        }
    }

    pub fn play(&mut self) {
        let mut player = 0;
        let mut die = DeterministicDie::new(0);

        loop {
            let dist = die.next().unwrap() + die.next().unwrap() + die.next().unwrap();
            self.num_rolls += 3;
            self.positions[player] = (self.positions[player] + dist - 1) % 10 + 1;
            self.scores[player] += self.positions[player];

            if self.scores[player] >= 1000 {
                break;
            }

            player += 1;
            player %= 2;
        }
    }

    #[must_use]
    pub fn scores(&self) -> &Vec<u32> {
        &self.scores
    }

    #[must_use]
    pub fn num_rolls(&self) -> u32 {
        self.num_rolls
    }
}

#[derive(Debug)]
struct DeterministicDie {
    curr: u32,
}

impl DeterministicDie {
    const SIDES: u32 = 100;

    #[must_use]
    fn new(start: u32) -> Self {
        Self { curr: start }
    }
}

impl Iterator for DeterministicDie {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr %= Self::SIDES;
        self.curr += 1;
        Some(self.curr)
    }
}

#[derive(Debug)]
pub struct DiracGame {
    curr_player: usize,
    curr_prob: u64,
    positions: [i32; 2],
    scores: [i32; 2],
}

impl DiracGame {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let positions = s
            .trim()
            .split('\n')
            .map(|s| s.split(": ").nth(1).unwrap().parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let scores = [0; 2];

        Self {
            curr_player: 0,
            curr_prob: 0,
            positions,
            scores,
        }
    }

    pub fn play(&mut self) -> [u64; 2] {
        let mut memo = HashMap::new();
        self.play_inner(&mut memo)
    }

    fn play_inner(&mut self, memo: &mut HashMap<State, [u64; 2]>) -> [u64; 2] {
        // The roll of three dice can be collapsed to one observable: the total distance to
        // move. Like how a roll of two dice can be described by a binomial distribution, a
        // roll of three dice can be described by a trinomial distribution.
        const PROBS: [(i32, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

        let mut wins = [0; 2];
        for (dist, prob) in PROBS {
            self.positions[self.curr_player] += dist;
            let score_add = (self.positions[self.curr_player] - 1) % 10 + 1;
            self.scores[self.curr_player] += score_add;
            self.curr_prob *= prob;

            if self.scores[self.curr_player] >= 21 {
                wins[self.curr_player] += prob;
            } else {
                self.curr_player = (self.curr_player + 1) % 2;

                let state = State {
                    player: self.curr_player,
                    positions: self.positions,
                    scores: self.scores,
                };

                if !memo.contains_key(&state) {
                    let wins = self.play_inner(memo);
                    memo.insert(state.clone(), wins);
                }
                let curr_wins = memo.get(&state).unwrap();
                wins[0] += prob * curr_wins[0];
                wins[1] += prob * curr_wins[1];

                self.curr_player = (self.curr_player + 1) % 2;
            }

            // Undo changes for the next iteration
            self.curr_prob -= prob;
            self.scores[self.curr_player] -= score_add;
            self.positions[self.curr_player] -= dist;
        }
        wins
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    player: usize,
    positions: [i32; 2],
    scores: [i32; 2],
}
