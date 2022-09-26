use std::{char, fmt, ops};

#[derive(Debug, Copy, Clone)]
pub struct Letter {
    value: u32,
}

impl From<u32> for Letter {
    fn from(val: u32) -> Self {
        Letter { value: val % 26}
    }
}

impl TryFrom<char> for Letter {
    type Error = char;
    fn try_from(letter: char) -> Result<Self, Self::Error> {
        if letter.is_ascii_alphabetic() {
            let ascii: u32 = letter.into();
            let return_val: Letter = ((ascii - 1) % 32).into();
            Ok(return_val)
        } else {
            Err(letter)
        }
    }
}

impl From<Letter> for char {
    fn from(my_letter: Letter) -> Self {
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let letter_num = my_letter.value.try_into().expect("Value should be between 0-26");
        letters.chars().nth(letter_num).expect("Letter is always valid")
    }
} 

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl ops::Add<Letter> for Letter {
    type Output = Letter;

    fn add(self, rhs: Letter) -> Self::Output {
        let rhs = rhs.value;
        let lhs = self.value;
        let val = (lhs + rhs) % 26;
        Letter { value: val }
    }
}

impl ops::Sub<Letter> for Letter {
    type Output = Letter;

    fn sub(self, rhs: Letter) -> Self::Output {
        let rhs = rhs.value % 26;
        let lhs = self.value;
        let val = (26 + lhs - rhs) % 26;
        Letter { value: val }
    }
}
