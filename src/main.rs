use std::io::{self, Read};
mod z26;
mod rule;

fn get_plain_text() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Couldnt read from stdin");
    buffer
}

fn main() {
    let plain_text = get_plain_text();
    let letter_rule = |letter| rule::shift(letter, 3);
    let encryption_rule =  rule::letter_map(letter_rule);
    let encrypted_text = encryption_rule(plain_text);
    println!("{}",encrypted_text);
}
