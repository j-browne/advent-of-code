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
    #[must_use]
    pub fn new(width: usize, height: usize, data: Vec<u32>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    #[must_use]
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

    #[must_use]
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
        write!(f, "\u{2554}")?;
        for _ in 0..self.width {
            write!(f, "\u{2550}")?;
        }
        writeln!(f, "\u{2557}")?;

        for i in 0..self.height {
            write!(f, "\u{2551}")?;
            for j in 0..self.width {
                match self.data[self.width * i + j] {
                    0 => write!(f, "\u{2588}")?,
                    1 => write!(f, " ")?,
                    _ => write!(f, "\u{2591}")?,
                };
            }
            writeln!(f, "\u{2551}")?;
        }

        write!(f, "\u{255a}")?;
        for _ in 0..self.width {
            write!(f, "\u{2550}")?;
        }
        write!(f, "\u{255d}")?;

        Ok(())
    }
}
