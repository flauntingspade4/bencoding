use crate::{
    err::{DeBencodingError::*, Result},
    BenCodeType::{self, *},
};

macro_rules! impl_bencoding {
    ($type:ty) => {
        impl BenCodeAble for $type {
            type Output = $type;

            fn bencode(&self) -> String {
                return format!("i{}e", self);
            }
            fn de_bencode(d: &mut Deserializer) -> Result<Self::Output> {
                let number = match d.input.find('e') {
                    Some(count) => match d.input.get(1..count) {
                        Some(t) => match t.parse::<Self>() {
                            Ok(number) => {
                                d.input = &d.input[1 + count..];
                                number
                            }
                            Err(_) => return Err(ParseIntError),
                        },
                        None => return Err(TrailingCharacters),
                    },
                    None => return Err(NoFoundClosingDeliminator),
                };
                return Ok(number);
            }
        }
    };
}

/// A trait describing a type's ability to be bencoded
/// implemented for strings, integers, and vec of the
/// aforementioned types
pub trait BenCodeAble {
    /// The output of [de_bencode](#tymethod.de_bencode)
    type Output;

    /// Turn self into a string
    fn bencode(&self) -> String;

    /// Turn a string into self
    fn de_bencode(d: &mut Deserializer) -> Result<Self::Output>;
}

impl<T: BenCodeAble<Output = T>> BenCodeAble for BenCodeType<T> {
    type Output = BenCodeType<T>;

    fn bencode(&self) -> String {
        match self {
            BenCodedString(s) => return s.bencode(),
            BenCodedInt(i) => return i.bencode(),
            BenCodedList(l) => return l.bencode(),
        }
    }

    fn de_bencode(d: &mut Deserializer) -> Result<Self::Output> {
        use BenCodeType::*;

        match d.input.chars().next() {
            Some(c) => match c {
                'i' => return Ok(BenCodedInt(i64::de_bencode(d)?)),
                'l' => return Ok(BenCodedList(Vec::<T>::de_bencode(d)?)),
                _ => return Ok(BenCodedString(String::de_bencode(d)?)),
            },
            None => return Err(TrailingCharacters),
        }
    }
}

impl BenCodeAble for String {
    type Output = String;

    fn bencode(&self) -> String {
        return format!("{}:{}", self.len(), self);
    }
    fn de_bencode(d: &mut Deserializer) -> Result<Self::Output> {
        // First, find the colon location (And check it exists)
        match d.input.find(':') {
            Some(count) => {
                // Split so it's "4" and ":test", at the location where ':' is found
                let (length, rest_of_string) = d.input.split_at(count);
                // Parse "4" into 4, so we know how many characters to take
                let len: usize = match length.parse::<usize>() {
                    // Add one to help ignore the added space from ':'
                    Ok(t) => t + 1,
                    Err(_) => return Err(ParseIntError),
                };
                // Make to_return between the colon, and the end of the second string
                let to_return = String::from(&rest_of_string[1..len]);
                // Remove the used string
                d.input = &d.input[count + len..];
                return Ok(to_return);
            }
            None => return Err(NoFoundColon),
        }
    }
}

impl BenCodeAble for &str {
    type Output = String;

    fn bencode(&self) -> String {
        return format!("{}:{}", self.len(), self);
    }
    fn de_bencode(d: &mut Deserializer) -> Result<Self::Output> {
        // First, find the colon location (And check it exists)
        match d.input.find(':') {
            Some(count) => {
                // Split so it's "4" and ":test", at the location where ':' is found
                let (length, rest_of_string) = d.input.split_at(count);
                // Parse "4" into 4, so we know how many characters to take
                let len: usize = match length.parse::<usize>() {
                    // Add one to help ignore the added space from ':'
                    Ok(t) => t + 1,
                    Err(_) => return Err(ParseIntError),
                };
                // Make to_return between the colon, and the end of the second string
                let to_return = String::from(&rest_of_string[1..len]);
                // Remove the used string
                d.input = &d.input[count + len..];
                return Ok(to_return);
            }
            None => return Err(NoFoundColon),
        }
    }
}

impl_bencoding!(i64);
impl_bencoding!(i32);
impl_bencoding!(i16);
impl_bencoding!(i8);

impl_bencoding!(u32);
impl_bencoding!(u16);
impl_bencoding!(u8);

impl<T: BenCodeAble<Output = T>> BenCodeAble for Vec<T> {
    type Output = Vec<T>;

    fn bencode(&self) -> String {
        let mut to_return = String::from("l");
        for item in self.iter() {
            to_return += &item.bencode();
        }
        to_return += "e";
        to_return
    }
    fn de_bencode(d: &mut Deserializer) -> Result<Self::Output> {
        if d.input == "le" {
            d.input = "";
            return Ok(Vec::new());
        }
        println!("S at beginning \"{}\"", d.input);
        let mut to_return = Vec::new();
        match d.next_char() {
            Ok(c) => {
                if c != 'l' {
                    return Err(NoFoundOpeningDeliminator);
                }
            }
            Err(_) => {
                return Err(EoF);
            }
        }
        while let Ok(c) = d.peek_char() {
            if c == 'e' {
                d.next_char().unwrap();
                return Ok(to_return);
            }
            let item = T::de_bencode(d)?;
            to_return.push(item);
        }
        Ok(to_return)
    }
}

pub struct Deserializer<'de> {
    input: &'de str,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}

/// Converts a &str to T, where T implements [BenCodeAble](../bencoding/trait.BenCodeAble.html)
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: BenCodeAble<Output = T>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::de_bencode(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        println!("Trailing chracters \"{}\"", deserializer.input);
        Err(TrailingCharacters)
    }
}

impl<'de> Deserializer<'de> {
    // Look at the first character in the input without consuming it.
    fn peek_char(&mut self) -> Result<char> {
        self.input.chars().next().ok_or(EoF)
    }

    // Consume the first character in the input.
    fn next_char(&mut self) -> Result<char> {
        let ch = self.peek_char()?;
        self.input = &self.input[ch.len_utf8()..];
        Ok(ch)
    }
}
