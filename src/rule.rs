use crate::z26::Letter;

pub struct Rule {
    encrypt: dyn Fn(String) -> String,
}

fn apply_letter_rule<F> (func: F, c: char) -> char 
where
    F: FnOnce(Letter) -> Letter 
{
    Letter::try_from(c)
        .map(func)
        .map(char::from)
        .unwrap_or(c)
}

pub fn letter_map<F>(func: F) -> impl Fn(String) -> String
where
    F: Fn(Letter) -> Letter
{
    let char_map = move |c: char| apply_letter_rule(&func, c);
    let str_map = move |s: String| -> String {s.chars().map(&char_map).collect::<String>()};
    str_map
}

pub fn shift(letter: Letter, key: u32) -> Letter {
    letter + key.into()
}