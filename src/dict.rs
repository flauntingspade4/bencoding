use core::fmt::{Debug, Display};

use crate::{
    err::{DeBencodingError::*, Result},
    BenCodeAble,
};

#[macro_export]
macro_rules! dict {
    ($key:expr, $value:expr) => {{
        let mut dict = Dict::new();
        dict.data.push(($key, $value));
        dict
    }};
    ($($key:expr, $value:expr),*) => {{
        let mut dict = Dict::new();
        $( dict.data.push(($key, $value)); )*
        dict
    }};
}

/// A simple dictionary for rust, just a wrapper
/// around Vec<(String, T)>. It's worth mentioning
/// that [de_bencode](../bencoding/trait.BenCodeAble.html#tymethod.de_bencode)ing this Dict will automatically
/// sort it for you.
pub struct Dict<T>
where
    T: BenCodeAble + Clone,
{
    pub data: Vec<(String, T)>,
}

impl<T> Dict<T>
where
    T: BenCodeAble<Output = T> + Clone,
{
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }
    fn ben_sort(&mut self) {
        self.data.sort_by(|first, second| first.0.cmp(&second.0));
    }
}

impl<T> BenCodeAble for Dict<T>
where
    T: BenCodeAble<Output = T> + Clone,
{
    type Output = Dict<T>;

    fn bencode(&self) -> String {
        let mut to_return = String::with_capacity(2 + self.data.len() * 2);
        let mut data = self.data.clone();
        data.sort_by(|first, second| first.0.cmp(&second.0));
        to_return.push('d');
        for item in data.iter() {
            to_return.push_str(&format!("{}{}", item.0.bencode(), item.1.bencode()));
        }
        to_return.push('e');
        to_return
    }

    fn de_bencode(d: &mut crate::bencode::Deserializer) -> Result<Self::Output> {
        if d.input == "de" {
            return Ok(Self::new());
        }
        let mut to_return = Self::new();
        match d.next_char() {
            Ok(c) => {
                if c != 'd' {
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
            let key = String::de_bencode(d)?;
            let item = T::de_bencode(d)?;
            to_return.data.push((key, item));
        }
        to_return.ben_sort();
        Ok(to_return)
    }
}

impl<T: BenCodeAble<Output = T> + Clone + Display> core::fmt::Display for Dict<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ ")?;

        for i in self.data.iter() {
            write!(f, "\"{}\" => {}", i.0, i.1)?;
        }
        write!(f, " }}")?;

        Ok(())
    }
}

impl<T: BenCodeAble<Output = T> + Clone + Debug> core::fmt::Debug for Dict<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ ")?;

        for i in self.data.iter() {
            write!(f, "\"{}\" => {:?}", i.0, i.1)?;
        }
        write!(f, " }}")?;

        Ok(())
    }
}

impl<T: BenCodeAble<Output = T> + Clone + PartialEq> PartialEq for Dict<T> {
    fn eq(&self, other: &Self) -> bool {
        for (index, item) in self.data.iter().enumerate() {
            if item.0 != other.data[index].0 || item.1 != other.data[index].1 {
                return false;
            }
        }
        return true;
    }
}
