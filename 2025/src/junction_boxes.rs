use std::{collections::HashSet, mem};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(pub u32, pub u32, pub u32);

impl Point {
    #[must_use]
    fn new(s: &str) -> Self {
        let mut coords = s.split(',').map(|x| x.parse().unwrap());
        Self(
            coords.next().unwrap(),
            coords.next().unwrap(),
            coords.next().unwrap(),
        )
    }

    fn distance(self, other: Self) -> f64 {
        f64::sqrt(
            (f64::from(self.0) - f64::from(other.0)).powi(2)
                + (f64::from(self.1) - f64::from(other.1)).powi(2)
                + (f64::from(self.2) - f64::from(other.2)).powi(2),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Circuits {
    boxes: Vec<Point>,
    circuits: Vec<HashSet<usize>>,
}

impl Circuits {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let boxes = s.lines().map(Point::new).collect();

        Self {
            boxes,
            circuits: Vec::new(),
        }
    }

    fn get_sorted_index_pairs(&self) -> Vec<(usize, usize)> {
        let mut pairs = (0..self.boxes.len())
            .flat_map(|i| ((i + 1)..self.boxes.len()).map(move |j| (i, j)))
            .collect::<Vec<_>>();

        pairs.sort_by(|(i0, j0), (i1, j1)| {
            self.boxes[*i0]
                .distance(self.boxes[*j0])
                .partial_cmp(&self.boxes[*i1].distance(self.boxes[*j1]))
                .unwrap()
        });

        pairs
    }

    pub fn connect(&mut self, num_to_connect: usize) {
        let pairs = self.get_sorted_index_pairs();

        for (i, j) in pairs.into_iter().take(num_to_connect) {
            self.connect_inner(i, j);
        }
    }

    #[must_use]
    pub fn get_last_connection(mut self) -> (Point, Point) {
        let pairs = self.get_sorted_index_pairs();

        for (i, j) in pairs {
            self.connect_inner(i, j);
            if self.circuits.len() == 1 && self.circuits[0].len() == self.boxes.len() {
                return (self.boxes[i], self.boxes[j]);
            }
        }

        unreachable!("everything should be connected after connecting every pair");
    }

    fn connect_inner(&mut self, i: usize, j: usize) {
        match (self.get_circuit_of(i), self.get_circuit_of(j)) {
            (Some(circuit_idx_1), Some(circuit_idx_2)) => {
                // if i and j are in the same circuit already, don't do anything
                // if they are in different circuits, combine them
                if circuit_idx_1 != circuit_idx_2 {
                    // take the data for circuit_2 from the list of circuits, but don't remove
                    // it until we insert the data into circuit_1. We do this so that
                    // circuit_idx_1 is still valid
                    let mut circuit_2 = HashSet::new();
                    mem::swap(&mut circuit_2, &mut self.circuits[circuit_idx_2]);
                    self.circuits[circuit_idx_1].extend(circuit_2);
                    self.circuits.remove(circuit_idx_2);
                }
            }
            (Some(circuit_idx), None) => {
                self.circuits[circuit_idx].insert(j);
            }
            (None, Some(circuit_idx)) => {
                self.circuits[circuit_idx].insert(i);
            }
            (None, None) => {
                let mut set = HashSet::new();
                set.insert(i);
                set.insert(j);
                self.circuits.push(set);
            }
        }
    }

    fn get_circuit_of(&self, box_idx: usize) -> Option<usize> {
        for (circuit_idx, circuit) in self.circuits.iter().enumerate() {
            if circuit.contains(&box_idx) {
                return Some(circuit_idx);
            }
        }

        None
    }

    #[must_use]
    pub fn circuit_sizes(&self) -> Vec<usize> {
        self.circuits.iter().map(HashSet::len).collect()
    }
}
