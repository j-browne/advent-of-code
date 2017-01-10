extern crate pcre;

use pcre::Pcre;
use std::io::BufRead;

fn supports_ssl(s: &str) -> bool {
    let mut re_sections: Pcre = Pcre::compile(r"(\[[a-z]+\]|[a-z]+)").unwrap();
    let mut re_aba: Pcre = Pcre::compile(r"([a-z])(?=(?!\1)([a-z])\1)").unwrap();

    let mut v_aba = Vec::<(u8, u8)>::new();
    let mut v_bab = Vec::<(u8, u8)>::new();

    for m_section in re_sections.matches(s) {
        let section = m_section.group(0);
        for m_aba in re_aba.matches(m_section.group(0)) {
            // HACK: this only works for matches that are 3 in length
            // because of overlapping matches problems
            let aba = &section.as_bytes()[m_aba.group_start(0)..m_aba.group_start(0)+3];
            if section.as_bytes()[0] == b'[' {
                v_bab.push((aba[1], aba[2]));
            } else {
                v_aba.push((aba[1], aba[2]));
            }
        }
    }

    for aba in &v_aba {
        for bab in &v_bab {
            if aba.0 == bab.1 && aba.1 == bab.0 {
                return true;
            }
        }
    }

    false
}

fn main() {
    let stdin = ::std::io::stdin();
    let mut sum = 0;

    for line in stdin.lock().lines() {
        let text = line.unwrap();
        if supports_ssl(&text) {
            sum += 1;
        }
    }
    println!("{}", sum);
}
