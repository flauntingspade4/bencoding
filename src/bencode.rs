use crate::{
    err::DeBencodingError::{self, *},
    BenCodeType::{self, *},
};

type Result<T> = std::result::Result<T, DeBencodingError>;

macro_rules! impl_bencoding_neg {
    ($type:ty) => {
        impl BenCodeAble for $type {
            type Output = i64;

            fn bencode(&self) -> String {
                if self == &-0 {
                    return format!("i0e");
                } else {
                    return format!("i{}e", self);
                }
            }
            fn de_bencode(s: &mut String) -> Result<(Self::Output, usize)> {
                let _ = match s.find('e') {
                    Some(count) => match s.get(1..count) {
                        Some(t) => match t.parse::<Self>() {
                            Ok(number) => return Ok((number as i64, count + 1)),
                            Err(_) => return Err(ParseIntError),
                        },
                        None => return Err(EndOfString),
                    },
                    None => return Err(NoFoundClosingDeliminator),
                };
            }
        }
    };
}

macro_rules! impl_bencoding {
    ($type:ty) => {
        impl BenCodeAble for $type {
            type Output = i64;

            fn bencode(&self) -> String {
                return format!("i{}e", self);
            }
            fn de_bencode(s: &mut String) -> Result<(Self::Output, usize)> {
                let num = match s.get(1..s.len() - 1) {
                    Some(t) => t,
                    None => return Err(EndOfString),
                };
                match num.parse::<Self>() {
                    Ok(t) => return Ok((t as i64, s.len() - 1)),
                    Err(_) => return Err(ParseIntError),
                }
            }
        }
    };
}

pub trait BenCodeAble {
    type Output;

    fn bencode(&self) -> String;

    fn de_bencode(s: &mut String) -> Result<(Self::Output, usize)>;
}

impl<T: BenCodeAble + BenCodeAble<Output = T>> BenCodeAble for BenCodeType<T> {
    type Output = BenCodeType<T>;

    fn bencode(&self) -> String {
        match self {
            BenCodedString(s) => return s.bencode(),
            BenCodedInt(i) => return i.bencode(),
            BenCodedList(l) => return l.bencode(),
        }
    }

    fn de_bencode(s: &mut String) -> Result<(Self::Output, usize)> {
        use BenCodeType::*;

        match s.chars().next() {
            Some(c) => {
                match c {
                    'i' => {
                        let bencoded = i64::de_bencode(s)?;
                        return Ok((BenCodedInt(bencoded.0), bencoded.1));
                        // to_return = Ok(BenCodedInt(i64::de_bencode(s)?));
                    }
                    'l' => {
                        let bencoded = Vec::<T>::de_bencode(s)?;
                        return Ok((BenCodedList(bencoded.0), bencoded.1));
                    }
                    _ => {
                        let bencoded = String::de_bencode(s)?;
                        return Ok((BenCodedString(bencoded.0), bencoded.1));
                    }
                }
            }
            None => return Err(EndOfString),
        }
        /*if !s.is_empty() {
            return Err(EndOfString);
        }
        to_return*/
    }
}

impl BenCodeAble for String {
    type Output = String;

    fn bencode(&self) -> String {
        return format!("{}:{}", self.len(), self);
    }
    fn de_bencode(s: &mut String) -> Result<(Self::Output, usize)> {
        match s.find(':') {
            Some(count) => {
                let (length, rest_of_string) = s.split_at(count);
                let len: usize = match length.parse() {
                    Ok(t) => t,
                    Err(_) => return Err(ParseIntError),
                };
                return Ok((String::from(&rest_of_string[1..len + 1]), len + 2));
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
    fn de_bencode(s: &mut String) -> Result<(Self::Output, usize)> {
        match s.find(':') {
            Some(count) => {
                let (length, rest_of_string) = s.split_at(count);
                let len: usize = match length.parse() {
                    Ok(t) => t,
                    Err(_) => return Err(ParseIntError),
                };
                return Ok((String::from(&rest_of_string[1..len + 1]), len));
            }
            None => return Err(NoFoundColon),
        }
    }
}

impl_bencoding_neg!(i64);
impl_bencoding_neg!(i32);
impl_bencoding_neg!(i16);
impl_bencoding_neg!(i8);

impl_bencoding!(u32);
impl_bencoding!(u16);
impl_bencoding!(u8);

impl<T: BenCodeAble + BenCodeAble<Output = T>> BenCodeAble for Vec<T> {
    type Output = Vec<T>;

    fn bencode(&self) -> String {
        let mut to_return = String::from("l");
        for item in self.iter() {
            to_return += &item.bencode();
        }
        to_return += "e";
        to_return
    }
    fn de_bencode(s: &mut String) -> Result<(Self::Output, usize)> {
        println!("S at beginning \"{}\"", s);
        let mut to_return = Vec::new();
        //let mut s = match s.get(1..s.len() - 1) {
        let mut s = match s.get(1..) {
            Some(t) => String::from(t),
            None => return Err(EndOfString),
        };
        println!("S after stripping start \"{}\"", s);
        let mut counter = 0;
        while !s.is_empty() {
            let (item, length) = T::de_bencode(&mut s)?;
            counter += length;
            to_return.push(item);
            s = match s.get(length..) {
                Some(t) => String::from(t),
                None => return Err(WrongLengthOfString),
            };
        }
        Ok((to_return, counter))
    }
}
