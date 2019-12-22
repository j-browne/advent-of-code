use std::{
    borrow::Borrow,
    cmp::Ordering,
    ops::{Add, AddAssign, Index, IndexMut},
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Vector {
    data: [i32; 3],
}

impl Vector {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { data: [x, y, z] }
    }

    pub fn x(&self) -> i32 {
        self.data[0]
    }

    pub fn y(&self) -> i32 {
        self.data[1]
    }

    pub fn z(&self) -> i32 {
        self.data[2]
    }

    pub fn x_mut(&mut self) -> &mut i32 {
        &mut self.data[0]
    }

    pub fn y_mut(&mut self) -> &mut i32 {
        &mut self.data[1]
    }

    pub fn z_mut(&mut self) -> &mut i32 {
        &mut self.data[2]
    }

    pub fn data(self) -> [i32; 3] {
        self.data
    }

    pub fn data_ref(&self) -> &[i32; 3] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [i32; 3] {
        &mut self.data
    }
}

impl<T: Borrow<Vector>> Add<T> for Vector {
    type Output = Self;

    fn add(mut self, other: T) -> Self::Output {
        let other = other.borrow();
        for (s, o) in self.data.iter_mut().zip(other.data.iter()) {
            *s += *o;
        }

        self
    }
}

impl<T: Borrow<Vector>> AddAssign<T> for Vector {
    fn add_assign(&mut self, other: T) {
        let other = other.borrow();
        for (s, o) in self.data.iter_mut().zip(other.data.iter()) {
            *s += *o;
        }
    }
}

impl Index<usize> for Vector {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct NBody {
    positions: Vec<Vector>,
    velocities: Vec<Vector>,
}

impl NBody {
    pub fn with_positions(positions: Vec<Vector>) -> Self {
        let velocities = vec![Vector::default(); positions.len()];
        Self {
            positions,
            velocities,
        }
    }

    pub fn step(&mut self) {
        self.update_velocities();
        self.update_positions();
    }

    fn update_velocities(&mut self) {
        for (i, (vi, pi)) in self
            .velocities
            .iter_mut()
            .zip(self.positions.iter())
            .enumerate()
        {
            for (j, pj) in self.positions.iter().enumerate() {
                if i != j {
                    for (vi, (pi, pj)) in vi.data.iter_mut().zip(pi.data.iter().zip(pj.data.iter()))
                    {
                        match pi.cmp(pj) {
                            Ordering::Less => *vi += 1,
                            Ordering::Greater => *vi -= 1,
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn update_positions(&mut self) {
        for (v, p) in self.velocities.iter().zip(self.positions.iter_mut()) {
            *p += v;
        }
    }

    pub fn total_energy(&self) -> u32 {
        self.positions
            .iter()
            .zip(self.velocities.iter())
            .map(|(p, v)| {
                let pot = p.data.iter().map(|x| x.abs()).sum::<i32>();
                let kin = v.data.iter().map(|x| x.abs()).sum::<i32>();
                pot * kin
            })
            .sum::<i32>() as u32
    }
}
