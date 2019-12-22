use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SpaceImage {
    width: usize,
    height: usize,
    data: Vec<u32>,
}

impl SpaceImage {
    pub fn new(width: usize, height: usize, data: Vec<u32>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    pub fn occurrences_in_layers(&self) -> Vec<HashMap<u32, u32>> {
        self.data
            .chunks(self.width * self.height)
            .map(|layer| {
                layer.iter().fold(HashMap::new(), |mut map, x| {
                    *map.entry(*x).or_insert(0) += 1;
                    map
                })
            })
            .collect::<Vec<_>>()
    }

    pub fn render(self) -> RenderedImage {
        let SpaceImage {
            width,
            height,
            data,
        } = self;
        let mut image = vec![2; self.width * self.height];

        data.chunks(self.width * self.height).for_each(|layer| {
            layer.iter().enumerate().for_each(|(i, pixel)| {
                if image[i] == 2 {
                    image[i] = *pixel;
                }
            })
        });

        RenderedImage {
            width,
            height,
            data: image,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RenderedImage {
    width: usize,
    height: usize,
    data: Vec<u32>,
}

impl Display for RenderedImage {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "╔")?;
        for _ in 0..self.width {
            write!(f, "═")?;
        }
        writeln!(f, "╗")?;

        for i in 0..self.height {
            write!(f, "║")?;
            for j in 0..self.width {
                match self.data[self.width * i + j] {
                    0 => write!(f, "█")?,
                    1 => write!(f, " ")?,
                    _ => write!(f, "░")?,
                };
            }
            writeln!(f, "║")?;
        }

        write!(f, "╚")?;
        for _ in 0..self.width {
            write!(f, "═")?;
        }
        write!(f, "╝")?;

        Ok(())
    }
}
