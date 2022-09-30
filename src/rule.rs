use std::env;

use crate::z26::Letter;

pub fn get_rule() -> Result<Box<dyn Fn(String) -> String>, &'static str> {
    let args: Vec<String> = env::args().collect();
    args.get(1)
        .map(|a| a.as_str())
        .ok_or("Must Specify Cypher Type")
        .and_then(|my_type| match my_type {
            "caeser" | "shift" | "r_caeser" | "r_shift" => if let "caeser" | "r_caeser" = my_type {
                Ok(3)
            } else {
                args.get(2)
                    .ok_or("Shift Requires Key")
                    .and_then(|s| s.parse::<i32>().map_err(|_| "Key Must Be Int"))
            }
            .and_then(|key| {
                Ok(if my_type.chars().nth(0).unwrap() == 'r' {
                    key * -1
                } else {
                    key
                })
            })
            .and_then(|key| {
                let key_func = move |c| -> Letter { shift(c, key) };
                Ok(letter_map(Box::new(key_func)))
            }),
            "affine" => {
                // let [a,b]] = <[&str; 3]>&args[2..=3];
                args.get(2..=3)
                    .ok_or("Affine Requires 2 Ints as key")
                    .and_then(|num_strings| {
                        num_strings
                            .iter()
                            .map(|s| s.parse::<i32>().map_err(|_| "Keys Must Be Ints"))
                            .collect()
                    })
                    .and_then(|args: Vec<i32>| {
                        <[i32; 2]>::try_from(args).map_err(|_| "Unknown Error")
                    })
                    .and_then(|args| {
                        let [a, b] = args;
                        let key_func = move |c| -> Letter { affine(c, a, b) };
                        Ok(letter_map(key_func))
                    })
            },
            
            _ => Err("Unsupported Cypher"),
        })
}

pub fn letter_map<F: 'static>(func: F) -> Box<dyn Fn(String) -> String>
where
    F: Fn(Letter) -> Letter,
{
    let char_map = move |c: char| Letter::try_from(c).map(&func).map(char::from).unwrap_or(c);
    let str_map = move |s: String| -> String { s.chars().map(&char_map).collect::<String>() };
    Box::new(str_map)
}

pub fn shift(letter: Letter, key: i32) -> Letter {
    letter + key.into()
}

pub fn affine(letter: Letter, a: i32, b: i32) -> Letter {
    letter * a.into() + b.into()
}


