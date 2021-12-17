#[derive(Debug)]
pub enum Packet {
    Literal {
        version: u64,
        ty: u64,
        value: u64,
    },
    Operation {
        version: u64,
        ty: u64,
        subs: Vec<Packet>,
    },
}

impl Packet {
    #[must_use]
    pub fn from_hex_str(s: &str) -> Self {
        let bits = s
            .split("")
            .filter_map(|x| u8::from_str_radix(x, 16).ok())
            .flat_map(|x| (0..4).map(move |i| x >> (3 - i) & 1))
            .collect::<Vec<_>>();
        Self::from_bits(&bits).0
    }

    #[must_use]
    pub fn from_bits(bits: &[u8]) -> (Self, usize) {
        //eprintln!("{:?}", bits);
        let version = from_be_bits(&bits[0..3]);
        let ty = from_be_bits(&bits[3..6]);
        let mut read = 6;

        if ty == 4 {
            let mut value = 0;
            let mut bits = &bits[6..];

            loop {
                value <<= 4;
                value += from_be_bits(&bits[1..5]);
                read += 5;

                if bits[0] == 0 {
                    break;
                }
                bits = &bits[5..];
            }

            (Self::Literal { version, ty, value }, read)
        } else {
            let len_ty = bits[6];
            read += 1;

            let subs = match len_ty {
                0 => {
                    let num_bits = usize::try_from(from_be_bits(&bits[7..22])).unwrap();
                    let mut bits = &bits[22..(22 + num_bits)];
                    read += 15;
                    let mut subs = Vec::new();

                    while !bits.is_empty() {
                        let (sub, n) = Packet::from_bits(bits);
                        subs.push(sub);
                        bits = &bits[n..];
                        read += n;
                    }

                    subs
                }
                1 => {
                    let num_subs = usize::try_from(from_be_bits(&bits[7..18])).unwrap();
                    let mut bits = &bits[18..];
                    read += 11;
                    let mut subs = Vec::new();

                    for _ in 0..num_subs {
                        let (sub, n) = Packet::from_bits(bits);
                        subs.push(sub);
                        bits = &bits[n..];
                        read += n;
                    }

                    subs
                }
                _ => unreachable!(),
            };

            (Self::Operation { version, ty, subs }, read)
        }
    }

    #[must_use]
    pub fn version_sum(&self) -> u64 {
        match self {
            Self::Literal { version, .. } => *version,
            Self::Operation { version, subs, .. } => {
                version + subs.iter().map(Self::version_sum).sum::<u64>()
            }
        }
    }

    #[must_use]
    pub fn value(&self) -> u64 {
        match self {
            Self::Literal { value, .. } => *value,
            Self::Operation { ty, subs, .. } => match ty {
                0 => subs.iter().map(Self::value).sum(),
                1 => subs.iter().map(Self::value).product(),
                2 => subs.iter().map(Self::value).min().unwrap(),
                3 => subs.iter().map(Self::value).max().unwrap(),
                5 => {
                    if subs[0].value() > subs[1].value() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if subs[0].value() < subs[1].value() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if subs[0].value() == subs[1].value() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

fn from_be_bits(bits: &[u8]) -> u64 {
    let n = bits.len() - 1;
    bits.iter()
        .enumerate()
        .map(|(i, b)| u64::try_from(*b).unwrap() << (n - i))
        .sum()
}
