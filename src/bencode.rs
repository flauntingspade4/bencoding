use crate::{
    err::{DeBencodingError::*, Result},
    // BenCodeType::{self, *},
};

macro_rules! impl_bencoding {
    ($($type:ty),*) => {
        $(impl<'a, 'de: 'a> BenCodeAble<'a, 'de> for $type {
            type Output = $type;

            fn bencode(&self) -> String {
                return format!("i{}e", self);
            }
            fn de_bencode(d: &'de mut Deserializer) -> Result<Self::Output> {
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
        })*
    };
}

impl_bencoding!(i64, i32, i16, i8);

impl_bencoding!(u64, u32, u16, u8);

/// A trait describing a type's ability to be bencoded
/// implemented for strings, integers, and vec of the
/// aforementioned types
pub trait BenCodeAble<'a, 'de: 'a> {
    /// The output of [de_bencode](#tymethod.de_bencode)
    type Output;

    /// Turn self into a string
    fn bencode(&self) -> String;

    /// Turn a string into self
    fn de_bencode(d: &'de mut Deserializer) -> Result<Self::Output>;
}

impl<'a, 'de: 'a> BenCodeAble<'a, 'de> for String {
    type Output = String;

    fn bencode(&self) -> String {
        return format!("{}:{}", self.len(), self);
    }
    fn de_bencode(d: &'de mut Deserializer) -> Result<Self::Output> {
        match d.input.find(':') {
            Some(count) => {
                let (length, rest_of_string) = d.input.split_at(count);
                let len: usize = match length.parse::<usize>() {
                    // Add one to help ignore the added space from ':'
                    Ok(t) => t + 1,
                    Err(_) => return Err(ParseIntError),
                };
                // Make to_return between the colon, and the end of the second string
                let to_return = String::from(&rest_of_string[1..len]);
                // Remove the used string
                d.input = &d.input[count + len..];
                Ok(to_return)
            }
            None => Err(NoFoundColon),
        }
    }
}

impl<'a, 'de: 'a> BenCodeAble<'a, 'de> for &str {
    type Output = &'de str;

    fn bencode(&self) -> String {
        return format!("{}:{}", self.len(), self);
    }
    fn de_bencode<'b>(d: &'de mut Deserializer) -> Result<Self::Output> {
        match d.input.find(':') {
            Some(count) => {
                let (length, rest_of) = d.input.split_at(count);
                let len: usize = match &length.parse::<usize>() {
                    // Add one to help ignore the added space from ':'
                    Ok(t) => t + 1,
                    Err(_) => return Err(ParseIntError),
                };
                // Make to_return between the colon, and the end of the second string
                let to_return = &rest_of[1..len];
                // Remove the used string
                d.input = &d.input[count + len..];
                Ok(to_return)
            }
            None => Err(NoFoundColon),
        }
    }
}

impl<'a, 'de: 'a, T: BenCodeAble<'a, 'de, Output = T>> BenCodeAble<'a, 'de> for Vec<T> {
    type Output = Vec<T>;

    fn bencode(&self) -> String {
        let mut to_return = String::from("l");
        for item in self.iter() {
            to_return += &item.bencode();
        }
        to_return += "e";
        to_return
    }
    fn de_bencode(d: &'de mut Deserializer) -> Result<Self::Output> {
        if d.input == "le" {
            d.input = "";
            return Ok(Vec::new());
        }
        let mut to_return = Vec::new();
        match d.next_char() {
            Ok(c) => {
                if c != 'l' {
                    return Err(NoFoundOpeningDeliminator);
                }
            }
            Err(_) => {
                return Err(Eof);
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
    pub input: &'de str,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}

/// Converts a &str to T, where T implements [BenCodeAble](../bencoding/trait.BenCodeAble.html)
pub fn from_str<'a, 'de: 'a, T>(s: &'de str) -> Result<T>
where
    T: BenCodeAble<'a, 'de, Output = T>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::de_bencode(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(TrailingCharacters)
    }
}

impl<'de> Deserializer<'de> {
    // Look at the first character in the input without consuming it.
    pub fn peek_char(&mut self) -> Result<char> {
        self.input.chars().next().ok_or(Eof)
    }

    // Consume the first character in the input.
    pub fn next_char(&mut self) -> Result<char> {
        let ch = self.peek_char()?;
        self.input = &self.input[ch.len_utf8()..];
        Ok(ch)
    }
}
