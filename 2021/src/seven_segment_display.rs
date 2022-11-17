use std::collections::HashMap;

#[derive(Debug)]
pub struct Display {
    signals: Vec<Signals>,
    output: Vec<Signals>,
}

impl Display {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.split(" | ");
        let signals = it
            .next()
            .unwrap()
            .split_whitespace()
            .map(Signals::new)
            .collect();
        let output = it
            .next()
            .unwrap()
            .split_whitespace()
            .map(Signals::new)
            .collect();
        Self { signals, output }
    }

    #[must_use]
    pub fn output_uniques(&self) -> usize {
        self.output
            .iter()
            .filter(|x| x.len() == 2 || x.len() == 4 || x.len() == 3 || x.len() == 7)
            .count()
    }

    #[must_use]
    pub fn output_value(&self) -> u32 {
        let mut signal_to_num = HashMap::new();
        let mut num_to_signal = HashMap::new();

        // 1 is the only signal with 2 segments
        let sig = self.signals.iter().find(|s| s.len() == 2).unwrap();
        signal_to_num.insert(sig, 1);
        num_to_signal.insert(1, sig);

        // 4 is the only signal with 4 segments
        let sig = self.signals.iter().find(|s| s.len() == 4).unwrap();
        signal_to_num.insert(sig, 4);
        num_to_signal.insert(4, sig);

        // 7 is the only signal with 3 segments
        let sig = self.signals.iter().find(|s| s.len() == 3).unwrap();
        signal_to_num.insert(sig, 7);
        num_to_signal.insert(7, sig);

        // 8 is the only signal with 7 segments
        let sig = self.signals.iter().find(|s| s.len() == 7).unwrap();
        signal_to_num.insert(sig, 8);
        num_to_signal.insert(8, sig);

        // 6 is the only len 6 that doesn't contain all of 1
        let sig = self
            .signals
            .iter()
            .find(|s| s.len() == 6 && num_to_signal[&1].iter().any(|x| !s.contains(x)))
            .unwrap();
        signal_to_num.insert(sig, 6);
        num_to_signal.insert(6, sig);

        // 9 is the only len 6 that contains all of 4
        let sig = self
            .signals
            .iter()
            .find(|s| s.len() == 6 && num_to_signal[&4].iter().all(|x| s.contains(x)))
            .unwrap();
        signal_to_num.insert(sig, 9);
        num_to_signal.insert(9, sig);

        // 0 is the remaining len 6
        let sig = self
            .signals
            .iter()
            .find(|s| s.len() == 6 && s != &num_to_signal[&6] && s != &num_to_signal[&9])
            .unwrap();
        signal_to_num.insert(sig, 0);
        num_to_signal.insert(0, sig);

        // 3 is the only len 5 that contains all of 1
        let sig = self
            .signals
            .iter()
            .find(|s| s.len() == 5 && num_to_signal[&1].iter().all(|x| s.contains(x)))
            .unwrap();
        signal_to_num.insert(sig, 3);
        num_to_signal.insert(3, sig);

        // 5 is the only len 5 that 6 contains all of
        let sig = self
            .signals
            .iter()
            .find(|s| s.len() == 5 && s.iter().all(|x| num_to_signal[&6].contains(x)))
            .unwrap();
        signal_to_num.insert(sig, 5);
        num_to_signal.insert(5, sig);

        // 2 is the remaining len 5
        let sig = self
            .signals
            .iter()
            .find(|s| s.len() == 5 && s != &num_to_signal[&3] && s != &num_to_signal[&5])
            .unwrap();
        signal_to_num.insert(sig, 2);
        num_to_signal.insert(2, sig);

        self.output.iter().fold(0, |x, s| 10 * x + signal_to_num[s])
    }
}
#[derive(Debug, PartialEq, Eq, Hash)]
#[allow(clippy::len_without_is_empty)]
pub struct Signals(pub [bool; 7]);

impl Signals {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut signals = [false; 7];
        for c in s.chars() {
            let idx = c as usize - 'a' as usize;
            signals[idx] = true;
        }
        Self(signals)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.iter().filter(|x| **x).count()
    }

    #[must_use]
    pub const fn contains(&self, c: char) -> bool {
        let idx = c as usize - 'a' as usize;
        self.0[idx]
    }

    pub fn iter(&self) -> impl Iterator<Item = char> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(n, x)| x.then(|| (b'a' + u8::try_from(n).unwrap()) as char))
    }
}
