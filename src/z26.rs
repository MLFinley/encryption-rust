use std::{char, fmt, ops};

#[derive(Debug, Copy, Clone)]
pub struct Letter {
    value: u8,
}

impl From<u32> for Letter {
    fn from(val: u32) -> Self {
        Letter {
            value: (val % 26).try_into().expect("26 < 2^8"),
        }
    }
}

impl From<i32> for Letter {
    fn from(val: i32) -> Self {
        Letter {
            value: (val % 26).try_into().expect("26 < 2^8"),
        }
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
        let letter_num = my_letter
            .value
            .try_into()
            .expect("Value should be between 0-26");
        letters
            .chars()
            .nth(letter_num)
            .expect("Letter is always valid")
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

impl ops::Mul<Letter> for Letter {
    type Output = Letter;

    fn mul(self, rhs: Letter) -> Self::Output {
        let rhs = rhs.value % 26;
        let lhs = self.value;
        let val = (lhs * rhs) % 26;
        Letter { value: val }
    }
}

impl ops::Neg for Letter {
    type Output = Letter;

    fn neg(self) -> Self::Output {
        Letter { value: 26 - self.value}
    }
}


#[derive(Debug, Clone, Copy)]
pub struct InvertableLetter {
    value: u8,
    inverse: u8,
}

impl TryFrom<Letter> for InvertableLetter {
    type Error = Letter;

    fn try_from(letter: Letter) -> Result<Self, Self::Error> {
        match letter.value {
            1 => Ok(InvertableLetter {
                value: 1,
                inverse: 25,
            }),
            3 => Ok(InvertableLetter {
                value: 3,
                inverse: 9,
            }),
            5 => Ok(InvertableLetter {
                value: 5,
                inverse: 21,
            }),
            7 => Ok(InvertableLetter {
                value: 7,
                inverse: 15,
            }),
            9 => Ok(InvertableLetter {
                value: 9,
                inverse: 3,
            }),
            11 => Ok(InvertableLetter {
                value: 11,
                inverse: 19,
            }),
            15 => Ok(InvertableLetter {
                value: 15,
                inverse: 7,
            }),
            17 => Ok(InvertableLetter {
                value: 17,
                inverse: 23,
            }),
            19 => Ok(InvertableLetter {
                value: 19,
                inverse: 11,
            }),
            21 => Ok(InvertableLetter {
                value: 21,
                inverse: 5,
            }),
            23 => Ok(InvertableLetter {
                value: 23,
                inverse: 17,
            }),
            25 => Ok(InvertableLetter {
                value: 25,
                inverse: 25,
            }),
            _ => Err(letter),
        }
    }
}

impl From<InvertableLetter> for Letter {
    fn from(invertable: InvertableLetter) -> Self {
        Letter { value : invertable.value }
    }
}

impl ops::Div<InvertableLetter> for Letter {
    type Output = Letter;

    fn div(self, rhs: InvertableLetter) -> Self::Output {
        self * Letter::from(u32::from(rhs.inverse))
    }
}

impl ops::Neg for InvertableLetter {
    type Output = InvertableLetter;

    fn neg(self) -> Self::Output {
        InvertableLetter { value: 26 - self.value, inverse: 26 - self.inverse}
    }
}