use std::slice::Iter;

use crate::z26::{InvertableLetter, Letter};

enum CypherType {
    Caeser(bool),
    Shift(bool),
    Affine(bool),
    Vigenere,
}

fn get_cypher(cypher_name: &String) -> Result<CypherType, String> {
    match cypher_name.as_str() {
        "caeser" => Ok(CypherType::Caeser(false)),
        "r_caeser" => Ok(CypherType::Caeser(true)),
        "shift" => Ok(CypherType::Shift(false)),
        "r_shift" => Ok(CypherType::Shift(true)),
        "affine" => Ok(CypherType::Affine(false)),
        "r_affine" => Ok(CypherType::Affine(true)),
        "vigenere" => Ok(CypherType::Vigenere),
        _ => Err(format!("Unsupported Cypher {}", cypher_name)),
    }
}

pub fn get_rule(args: Vec<String>) -> Result<Box<dyn Fn(String) -> String>, String> {
    args.get(1)
        .ok_or(String::from("Must Specify Cypher Type"))
        .and_then(|cypher_name| get_cypher(cypher_name))
        .and_then(|cypher_type| match cypher_type {
            CypherType::Caeser(reversed) | CypherType::Shift(reversed) => {
                if matches!(cypher_type, CypherType::Caeser(_)) {
                    Ok(3)
                } else {
                    args.get(2)
                        .ok_or(String::from("Shift Requires Key"))
                        .and_then(|s| {
                            s.parse::<i32>()
                                .map_err(|_| String::from("Key Must Be Int"))
                        })
                }
                .map(Letter::from)
                .map(|key| if reversed { -key } else { key })
                .map(|key| {
                    let key_func = move |c| shift(c, key);
                    letter_map(Box::new(key_func))
                })
            }

            CypherType::Affine(reversed) => args
                .get(2..=3)
                .ok_or(String::from("Affine Requires 2 Ints as key"))
                .and_then(|num_strings| {
                    num_strings
                        .iter()
                        .map(|s| {
                            s.parse::<i32>()
                                .map_err(|_| String::from("Keys Must Be Ints"))
                        })
                        .collect()
                })
                .map(|args: Vec<i32>| {
                    <[i32; 2]>::try_from(args).expect("We have already handeld this erro")
                })
                .map(|[a, b]| [Letter::from(a), Letter::from(b)])
                .and_then(|[a, b]| {
                    InvertableLetter::try_from(a)
                        .map_err(|_| String::from("First value must be invertible"))
                        .map(|a| (a, b))
                })
                .map(|(a, b)| {
                    if reversed {
                        [Letter::from(1) / -a, -b / a]
                    } else {
                        [Letter::from(a), b]
                    }
                })
                .map(|[a, b]| {
                    let key_func = move |c| affine(c, a, b);
                    letter_map(Box::new(key_func))
                }),
            CypherType::Vigenere => args
                .get(2)
                .ok_or(String::from("Vigenre requires a string key"))
                .and_then(|key| {
                    key.chars()
                        .map(Letter::try_from)
                        .collect::<Result<Vec<Letter>, char>>()
                        .map_err(|_| String::from("Key must only be letters"))
                })
                .map(|key| {
                    let str_fun: Box<dyn Fn(String) -> String> =
                        Box::new(move |s| vigenere(s, key.iter()));
                    str_fun
                }),
        })
}


fn letter_map<F: 'static>(func: F) -> Box<dyn Fn(String) -> String>
where
    F: Fn(Letter) -> Letter,
{
    let char_map = move |c: char| Letter::try_from(c).map(&func).map(char::from).unwrap_or(c);
    let str_map = move |s: String| -> String { s.chars().map(&char_map).collect::<String>() };
    Box::new(str_map)
}

fn shift(letter: Letter, key: Letter) -> Letter {
    letter + key
}

fn affine(letter: Letter, a: Letter, b: Letter) -> Letter {
    letter * a + b
}

fn vigenere(plain_text: String, key: Iter<Letter>) -> String {
    plain_text
        .chars()
        .map(Letter::try_from)
        .filter_map(|x| x.ok())
        .zip(key)
        .map(|(c, k)| c + *k)
        .map(char::from)
        .collect()
}
