extern crate pcre;

use pcre::Pcre;
use std::io::BufRead;

fn supports_tls(s: &str) -> bool {
    let mut re_sections: Pcre = Pcre::compile(r"(\[[a-z]+\]|[a-z]+)").unwrap();
    let mut re_abba: Pcre = Pcre::compile(r"([a-z])(?!\1)([a-z])\2\1").unwrap();

    let mut abba_out = false;

    for m_section in re_sections.matches(s) {
        let section = m_section.group(0);
        for _m_abba in re_abba.matches(m_section.group(0)) {
            //let abba = m_abba.group(0);
            if section.as_bytes()[0] == b'[' {
                return false;
            } else {
                abba_out = true;
            }
        }
    }

    abba_out
}

fn main() {
    let stdin = ::std::io::stdin();
    let mut sum = 0;

    for line in stdin.lock().lines() {
        let text = line.unwrap();
        if supports_tls(&text) {
            sum += 1;
        }
    }
    println!("{}", sum);
}
