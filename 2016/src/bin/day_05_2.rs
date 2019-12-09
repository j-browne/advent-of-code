extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() {
    let mut hasher = Md5::new();
    let id = b"wtnhxymk";

    let mut total = 0;
    let mut i = 0;
    let mut pass: [Option<char>; 8] = [None; 8];
    while total < 8 {
        let mut hash = [0u8; 16];
        hasher.input(id);
        hasher.input(i.to_string().as_bytes());
        hasher.result(&mut hash);

        let mut hash_str = String::new();
        for j in hash.iter() {
            hash_str.push_str(&format!("{:>02X}", j));
        }

        if hash_str.starts_with("00000") {
            let pos = (hash_str.as_bytes()[5] - b'0') as usize;
            if pos < 8 && pass[pos].is_none() {
                total += 1;
                pass[pos] = Some(hash_str.as_bytes()[6] as char);
            }
        }

        hasher.reset();
        i += 1;

        for j in 0..8 {
            print!("{}", pass[j].unwrap_or('_'));
        }
        print!("\r");
    }
    println!("");
}
