use std::io::{self, Read};

use crate::rule::get_rule;
mod z26;
mod rule;

fn get_plain_text() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Couldnt read from stdin");
    buffer
}



fn main() {
    let encryption_rule = match get_rule() {
        Ok(rule) => rule,
        Err(error) => {
            dbg!(error);
            std::process::exit(1)
        },
    };
    let plain_text = get_plain_text();
    let encrypted_text = encryption_rule(plain_text);
    println!("{}",encrypted_text);
}
