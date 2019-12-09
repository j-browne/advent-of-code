extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() {
    let mut hasher = Md5::new();
    let id = b"wtnhxymk";

    let mut total = 0;
    let mut i = 0;
    let mut pass = String::new();
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
            total += 1;
            pass.push(hash_str.as_bytes()[5] as char);
        }

        hasher.reset();
        i += 1;
    }
    println!("{}", pass);
}
