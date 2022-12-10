use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Movements {
    moves: Vec<Move>,
    len: usize,
}

impl Movements {
    #[must_use]
    pub fn with_len(s: &str, len: usize) -> Self {
        let moves = s.lines().map(Move::new).collect();

        Self { moves, len }
    }

    #[must_use]
    pub fn tail_locations(&self) -> HashSet<(i32, i32)> {
        let mut locs = HashSet::new();
        let mut knots = vec![(0, 0); self.len];

        for Move { dir, dist } in &self.moves {
            for _ in 0..*dist {
                move_head(&mut knots[0], *dir);
                for i in 1..self.len {
                    let (head, tail) = knots.split_at_mut(i);
                    let head = head.get(i - 1).unwrap();
                    let tail = tail.get_mut(0).unwrap();
                    move_tail(tail, *head);
                }
                locs.insert(*knots.last().unwrap());
            }
        }

        locs
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    dir: Dir,
    dist: usize,
}

impl Move {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.split_whitespace();
        let dir = Dir::new(it.next().unwrap());
        let dist = it.next().unwrap().parse().unwrap();

        Self { dir, dist }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    #[must_use]
    pub fn new(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "R" => Self::Right,
            "D" => Self::Down,
            "L" => Self::Left,
            d => panic!("unknown dir '{d}'"),
        }
    }
}

fn move_head(head: &mut (i32, i32), dir: Dir) {
    match dir {
        Dir::Up => *head = (head.0, head.1 - 1),
        Dir::Right => *head = (head.0 + 1, head.1),
        Dir::Down => *head = (head.0, head.1 + 1),
        Dir::Left => *head = (head.0 - 1, head.1),
    }
}

fn move_tail(tail: &mut (i32, i32), head: (i32, i32)) {
    let diff = ((head.0 - tail.0), (head.1 - tail.1));
    match diff {
        (1 | 0 | -1, 1 | 0 | -1) => {}
        _ => {
            *tail = (tail.0 + diff.0.signum(), tail.1 + diff.1.signum());
        }
    }
}
