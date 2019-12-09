extern crate pcre;

use pcre::Pcre;
use std::collections::HashMap;
use std::io::BufRead;
use Command::*;

#[derive(Debug, PartialEq)]
enum Id {
    Bin(u32),
    Bot(u32),
}

#[derive(Debug)]
#[allow(dead_code)]
enum Command {
    FromBin(Id, Id),
    FromBot(Id, Id, Id),
}

#[derive(Debug)]
struct Bot {
    chips: (Option<u32>, Option<u32>),
}

impl Bot {
    fn new() -> Bot {
        Bot {
            chips: (None, None),
        }
    }

    fn push(&mut self, v1: u32) {
        match self.chips {
            (None, None) => {
                self.chips = (Some(v1), None);
            }
            (Some(v2), None) => {
                if v1 > v2 {
                    self.chips = (Some(v2), Some(v1));
                } else {
                    self.chips = (Some(v1), Some(v2));
                }
            }
            (None, Some(_)) => {
                panic!("Bot chips not ordered");
            }
            (Some(_), Some(_)) => {
                panic!("Bot already has two chips");
            }
        }
    }

    fn clear(&mut self) {
        self.chips = (None, None);
    }

    fn num_chips(&self) -> u32 {
        match self.chips {
            (Some(_), Some(_)) => 2,
            (Some(_), None) => 1,
            (None, None) => 0,
            (None, Some(_)) => { panic!("Bot chips not ordered"); }
        }
    }
}

#[derive(Debug)]
struct Factory {
    bots: HashMap<u32, Bot>,
    commands: Vec<Command>,
    curr_comm: usize,
}

impl Factory {
    fn new() -> Factory {
        Factory {
            bots: HashMap::new(),
            commands: Vec::new(),
            curr_comm: 0,
        }
    }

    fn add_command(&mut self, c: &str) {
        let mut re_bin_bot = Pcre::compile(r"value ([0-9]+) goes to bot ([0-9]+)").unwrap();
        let mut re_bot_bin_bin = Pcre::compile(r"bot ([0-9]+) gives low to output ([0-9]+) and high to output ([0-9]+)").unwrap();
        let mut re_bot_bin_bot = Pcre::compile(r"bot ([0-9]+) gives low to output ([0-9]+) and high to bot ([0-9]+)").unwrap();
        let mut re_bot_bot_bin = Pcre::compile(r"bot ([0-9]+) gives low to bot ([0-9]+) and high to output ([0-9]+)").unwrap();
        let mut re_bot_bot_bot = Pcre::compile(r"bot ([0-9]+) gives low to bot ([0-9]+) and high to bot ([0-9]+)").unwrap();

        for m_comm in re_bin_bot.matches(c) {
            let chip = m_comm.group(1).parse::<u32>().unwrap();
            let dest = m_comm.group(2).parse::<u32>().unwrap();

            self.commands.push(FromBin(Id::Bin(chip), Id::Bot(dest)));
            let _ = self.bots.entry(dest).or_insert_with(Bot::new);
        }
        for m_comm in re_bot_bin_bin.matches(c) {
            let src = m_comm.group(1).parse::<u32>().unwrap();
            let dest_low = m_comm.group(2).parse::<u32>().unwrap();
            let dest_high = m_comm.group(3).parse::<u32>().unwrap();

            self.commands.push(FromBot(Id::Bot(src), Id::Bin(dest_low), Id::Bin(dest_high)));
            let _ = self.bots.entry(src).or_insert_with(Bot::new);
        }
        for m_comm in re_bot_bin_bot.matches(c) {
            let src = m_comm.group(1).parse::<u32>().unwrap();
            let dest_low = m_comm.group(2).parse::<u32>().unwrap();
            let dest_high = m_comm.group(3).parse::<u32>().unwrap();

            self.commands.push(FromBot(Id::Bot(src), Id::Bin(dest_low), Id::Bot(dest_high)));
            let _ = self.bots.entry(src).or_insert_with(Bot::new);
            let _ = self.bots.entry(dest_high).or_insert_with(Bot::new);
        }
        for m_comm in re_bot_bot_bin.matches(c) {
            let src = m_comm.group(1).parse::<u32>().unwrap();
            let dest_low = m_comm.group(2).parse::<u32>().unwrap();
            let dest_high = m_comm.group(3).parse::<u32>().unwrap();

            self.commands.push(FromBot(Id::Bot(src), Id::Bot(dest_low), Id::Bin(dest_high)));
            let _ = self.bots.entry(src).or_insert_with(Bot::new);
            let _ = self.bots.entry(dest_low).or_insert_with(Bot::new);
        }
        for m_comm in re_bot_bot_bot.matches(c) {
            let src = m_comm.group(1).parse::<u32>().unwrap();
            let dest_low = m_comm.group(2).parse::<u32>().unwrap();
            let dest_high = m_comm.group(3).parse::<u32>().unwrap();

            self.commands.push(FromBot(Id::Bot(src), Id::Bot(dest_low), Id::Bot(dest_high)));
            let _ = self.bots.entry(src).or_insert_with(Bot::new);
            let _ = self.bots.entry(dest_low).or_insert_with(Bot::new);
            let _ = self.bots.entry(dest_high).or_insert_with(Bot::new);
        }
    }

    fn step(&mut self) {
        match self.commands[self.curr_comm] {
            FromBin(ref src, ref dest) => {
                let can_take;
                let can_put;

                can_take = match src {
                    &Id::Bin(_) => {
                        true
                    }
                    &Id::Bot(_) => {
                        panic!("Using FromBin not from a Bin");
                    }
                };
                can_put = match dest {
                    &Id::Bot(id) => {
                        let bot = self.bots.get(&id).unwrap();
                        bot.num_chips() <= 1
                    }
                    &Id::Bin(_) => true,
                };


                if can_take && can_put {
                    let chip;

                    match src {
                        &Id::Bin(id) => {
                            chip = id;
                        }
                        &Id::Bot(_) => {
                            panic!("Using FromBin not from a Bin");
                        }
                    }
                    match dest {
                        &Id::Bot(id) => {
                            let bot = self.bots.get_mut(&id).unwrap();
                            bot.push(chip);
                        }
                        &Id::Bin(_) => { }
                    }
                }
            }
            FromBot(ref src, ref dest_low, ref dest_high) => {
                let can_take;
                let can_put_low;
                let can_put_high;

                can_take = match src {
                    &Id::Bot(id) => {
                        let bot = self.bots.get(&id).unwrap();
                        bot.num_chips() >= 2
                    }
                    &Id::Bin(_) => {
                        panic!("Using FromBot not from a Bot");
                    }
                };
                if dest_low == dest_high {
                    can_put_low = match dest_low {
                        &Id::Bot(id) => {
                            let bot = self.bots.get(&id).unwrap();
                            bot.num_chips() <= 0
                        }
                        &Id::Bin(_) => true,
                    };
                    can_put_high = can_put_low;
                } else {
                    can_put_low = match dest_low {
                        &Id::Bot(id) => {
                            let bot = self.bots.get(&id).unwrap();
                            bot.num_chips() <= 1
                        }
                        &Id::Bin(_) => true,
                    };
                    can_put_high = match dest_high {
                        &Id::Bot(id) => {
                            let bot = self.bots.get(&id).unwrap();
                            bot.num_chips() <= 1
                        }
                        &Id::Bin(_) => true,
                    };
                }


                if can_take && can_put_low && can_put_high {
                    let chips;

                    match src {
                        &Id::Bot(id) => {
                            let bot = self.bots.get_mut(&id).unwrap();
                            chips = bot.chips;
                            bot.clear();
                        }
                        &Id::Bin(_) => {
                            panic!("Using FromBot not from a Bot");
                        }
                    }
                    match dest_low {
                        &Id::Bot(id) => {
                            let bot = self.bots.get_mut(&id).unwrap();
                            bot.push(chips.0.unwrap());
                        }
                        &Id::Bin(_) => { }
                    }
                    match dest_high {
                        &Id::Bot(id) => {
                            let bot = self.bots.get_mut(&id).unwrap();
                            bot.push(chips.1.unwrap());
                        }
                        &Id::Bin(_) => { }
                    }
                } else {
                    // Clear src even if you can't place it? IDK why
                    match src {
                        &Id::Bot(id) => {
                            let bot = self.bots.get_mut(&id).unwrap();
                            bot.clear();
                        }
                        &Id::Bin(_) => {
                            panic!("Using FromBot not from a Bot");
                        }
                    }
                }
            }
        }
        self.curr_comm += 1;
        if self.curr_comm + 1 > self.commands.len() {
            self.curr_comm = 0;
        }
    }
}

fn main() {
    let mut factory = Factory::new();
    let stdin = ::std::io::stdin();

    for line in stdin.lock().lines() {
        factory.add_command(&line.unwrap());
    }

    'main: loop {
        factory.step();
        for (i, b) in &factory.bots {
            if b.chips == (Some(17), Some(61)) { 
                println!("{}", i);
                break 'main;
            }
        }
    }
}
