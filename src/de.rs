use std::{ops::Neg, str::FromStr};

use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;

use crate::err::{DeBencodingError, Result};

pub struct Deserializer<'de> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    input: &'de [u8],
}

impl<'de> Deserializer<'de> {
    // By convention, `Deserializer` constructors are named like `from_xyz`.
    // That way basic use cases are satisfied by something like
    // `serde_json::from_str(...)` while advanced use cases that require a
    // deserializer can make one with `serde_json::Deserializer::from_str(...)`.
    pub const fn from_str(input: &'de str) -> Self {
        Self::from_bytes(input.as_bytes())
    }

    pub const fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer { input }
    }
}

/// Attempts to read a value from a given string
///
/// # Errors
/// Fails if deserialization fails
pub fn from_str<'de, T>(s: &'de str) -> Result<T>
where
    T: Deserialize<'de>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(DeBencodingError::TrailingCharacters)
    }
}

/// A convenience function for building a deserializer
/// and deserializing a value of type `T` from bytes.
///
/// # Errors
/// Fails if deserialization fails
pub fn from_bytes<'de, T>(s: &'de [u8]) -> Result<T>
where
    T: de::Deserialize<'de>,
{
    let mut deserializer = Deserializer::from_bytes(s);
    let t = T::deserialize(&mut deserializer)?;

    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(DeBencodingError::TrailingCharacters)
    }
}

// SERDE IS NOT A PARSING LIBRARY. This impl block defines a few basic parsing
// functions from scratch. More complicated formats may wish to use a dedicated
// parsing library to help implement their Serde deserializer.
impl<'de> Deserializer<'de> {
    // Look at the first character in the input without consuming it.
    fn peek_char(&mut self) -> Result<&u8> {
        self.input.iter().next().ok_or(DeBencodingError::Eof)
    }

    // Consume the first character in the input.
    fn next_char(&mut self) -> Result<u8> {
        let ch = *self.peek_char()?;
        self.input = &self.input[1..];
        Ok(ch)
    }

    // Parse the JSON identifier `true` or `false`.
    fn parse_bool(&mut self) -> Result<bool> {
        panic!("This version of bencoding doesn't support bools");
    }

    // Parse a group of decimal digits as an unsigned integer of type T.
    //
    // This implementation is a bit too lenient, for example `001` is not
    // allowed in JSON. Also the various arithmetic operations can overflow and
    // panic or return bogus data. But it is good enough for example code!
    fn parse_unsigned<T>(&mut self) -> Result<T>
    where
        T: FromStr,
    {
        let number = match self.input.iter().position(|b| b == &b'e') {
            Some(count) => match self.input.get(1..count) {
                Some(v) => match std::str::from_utf8(v).unwrap().parse::<T>() {
                    Ok(number) => {
                        self.input = &self.input[1 + count..];
                        number
                    }
                    Err(_) => return Err(DeBencodingError::ParseIntError),
                },
                None => return Err(DeBencodingError::TrailingCharacters),
            },
            None => return Err(DeBencodingError::NoFoundClosingDeliminator),
        };
        Ok(number)
    }

    // Parse a possible minus sign followed by a group of decimal digits as a
    // signed integer of type T.
    fn parse_signed<T>(&mut self) -> Result<T>
    where
        T: Neg<Output = T> + FromStr + Neg<Output = T>,
    {
        if self.peek_char().unwrap() == &b'-' {
            self.next_char()?;
            Ok(-self.parse_unsigned()?)
        } else {
            self.parse_unsigned()
        }
    }

    // Not actually used, due to lifetime issues
    /*
    // Parse a string until the next '"' character.
    //
    // Makes no attempt to handle escape sequences.
    fn parse_string(&'de mut self) -> Result<&'de str> {
        match self.input.iter().position(|b| b == &b':') {
            Some(count) => {
                let (length, rest_of) = self.input.split_at(count);
                let len = match std::str::from_utf8(length).unwrap().parse::<usize>() {
                    // Add one to help ignore the added space from ':'
                    Ok(t) => t + 1,
                    Err(_) => return Err(DeBencodingError::ParseIntError),
                };
                // Make to_return between the colon, and the end of the second string
                let to_return = &rest_of[1..len];
                // Remove the used string
                self.input = &self.input[count + len..];
                Ok(std::str::from_utf8(to_return).unwrap())
            }
            None => Err(DeBencodingError::NoFoundColon),
        }
    }*/
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = DeBencodingError;

    // Look at the input data to decide what Serde data model type to
    // deserialize as. Not all data formats are able to support this operation.
    // Formats that support `deserialize_any` are known as self-describing.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.peek_char()? {
            &b'0' | &b'1' | &b'2' | &b'3' | &b'4' | &b'5' | &b'6' | &b'7' | &b'8' | &b'9' => {
                self.deserialize_str(visitor)
            }
            &b'i' => self.deserialize_i64(visitor),
            &b'l' => self.deserialize_seq(visitor),
            &b'd' => self.deserialize_map(visitor),
            _ => Err(DeBencodingError::UnexpectedCharType('a')),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    // The `parse_signed` function is generic over the integer type `T` so here
    // it is invoked with `T=i8`. The next 8 methods are similar.
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_signed()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_signed()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_signed()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_signed()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_unsigned()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_unsigned()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_unsigned()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_unsigned()?)
    }

    // Float parsing is stupidly hard.
    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // Float parsing is stupidly hard.
    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // The `Serializer` implementation on the previous page serialized chars as
    // single-character strings so handle that representation here.
    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Parse a string, check that it is one character, call `visit_char`.
        unimplemented!()
    }

    // Refer to the "Understanding deserializer lifetimes" page for information
    // about the three deserialization flavors of strings in Serde.
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let string = {
            match self.input.iter().position(|b| b == &b':') {
                Some(count) => {
                    let (length, rest_of) = self.input.split_at(count);
                    let len = match std::str::from_utf8(length).unwrap().parse::<usize>() {
                        // Add one to help ignore the added space from ':'
                        Ok(t) => t + 1,
                        Err(_) => return Err(DeBencodingError::ParseIntError),
                    };
                    // Make to_return between the colon, and the end of the second string
                    let to_return = &rest_of[1..len];
                    // Remove the used string
                    self.input = &self.input[count + len..];
                    Ok(std::str::from_utf8(to_return).unwrap())
                }
                None => Err(DeBencodingError::NoFoundColon),
            }
        }?;
        visitor.visit_borrowed_str(string)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    // The `Serializer` implementation on the previous page serialized byte
    // arrays as JSON arrays of bytes. Handle that representation here.
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let s = {
            match self.input.iter().position(|b| b == &b':') {
                Some(count) => {
                    let (length, rest_of) = self.input.split_at(count);
                    let len = match std::str::from_utf8(length).unwrap().parse::<usize>() {
                        // Add one to help ignore the added space from ':'
                        Ok(t) => t + 1,
                        Err(_) => return Err(DeBencodingError::ParseIntError),
                    };
                    // Make to_return between the colon, and the end of the second string
                    let to_return = &rest_of[1..len];
                    // Remove the used string
                    self.input = &self.input[count + len..];
                    Ok(to_return)
                }
                None => Err(DeBencodingError::NoFoundColon),
            }
        }?;
        visitor.visit_bytes(s.as_ref())
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // An absent optional is represented as the JSON `null` and a present
    // optional is represented as just the contained value.
    //
    // As commented in `Serializer` implementation, this is a lossy
    // representation. For example the values `Some(())` and `None` both
    // serialize as just `null`. Unfortunately this is typically what people
    // expect when working with JSON. Other formats are encouraged to behave
    // more intelligently if possible.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.input.starts_with(b"null") {
            self.input = &self.input["null".len()..];
            //self.input = self.input.split_off("null".len());
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    // In Serde, unit means an anonymous value containing no data.
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.input.starts_with(b"null") {
            self.input = &self.input["null".len()..];
            //self.input = self.input.split_off("null".len());
            visitor.visit_unit()
        } else {
            Err(DeBencodingError::ExpectedNull)
        }
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain. That means not
    // parsing anything other than the contained value.
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    // Deserialization of compound types like sequences and maps happens by
    // passing the visitor an "Access" object that gives it the ability to
    // iterate through the data contained in the sequence.
    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Parse the opening character of the sequence.
        if self.next_char()? == b'l' {
            // Give the visitor access to each element of the sequence.
            let value = visitor.visit_seq(&mut self)?;
            // Parse the closing character of the sequence.
            if self.next_char()? == b'e' {
                Ok(value)
            } else {
                Err(DeBencodingError::NoFoundClosingDeliminator)
            }
        } else {
            Err(DeBencodingError::NoFoundOpeningDeliminator)
        }
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently.
    //
    // As indicated by the length parameter, the `Deserialize` implementation
    // for a tuple in the Serde data model is required to know the length of the
    // tuple before even looking at the input data.
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    // Tuple structs look just like sequences in JSON.
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    // Much like `deserialize_seq` but calls the visitors `visit_map` method
    // with a `MapAccess` implementation, rather than the visitor's `visit_seq`
    // method with a `SeqAccess` implementation.
    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Parse the opening brace of the map.
        if self.next_char()? == b'd' {
            // Give the visitor access to each entry of the map.
            let value = visitor.visit_map(&mut self)?;
            // Parse the closing brace of the map.
            if self.next_char()? == b'e' {
                Ok(value)
            } else {
                Err(DeBencodingError::NoFoundClosingDeliminator)
            }
        } else {
            Err(DeBencodingError::NoFoundOpeningDeliminator)
        }
    }

    // Structs look just like maps in JSON.
    //
    // Notice the `fields` parameter - a "struct" in the Serde data model means
    // that the `Deserialize` implementation is required to know what the fields
    // are before even looking at the input data. Any key-value pairing in which
    // the fields cannot be known ahead of time is probably a map.
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        panic!("This version of bencoding doesn't support enums")
    }

    // An identifier in Serde is the type that identifies a field of a struct or
    // the variant of an enum. In JSON, struct fields and enum variants are
    // represented as strings. In other formats they may be represented as
    // numeric indices.
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    // Like `deserialize_any` but indicates to the `Deserializer` that it makes
    // no difference which `Visitor<'de>` method is called because the data is
    // ignored.
    //
    // Some deserializers are able to implement this more efficiently than
    // `deserialize_any`, for example by rapidly skipping over matched
    // delimiters without paying close attention to the data in between.
    //
    // Some formats are not able to implement this at all. Formats that can
    // implement `deserialize_any` and `deserialize_ignored_any` are known as
    // self-describing.
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

impl<'de> SeqAccess<'de> for Deserializer<'de> {
    type Error = DeBencodingError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.peek_char()? == &b'e' {
            return Ok(None);
        }
        // Deserialize an array element.
        seed.deserialize(self).map(Some)
    }
}

impl<'de> MapAccess<'de> for Deserializer<'de> {
    type Error = DeBencodingError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        // Check if there are no more entries.
        if self.peek_char()? == &b'e' {
            return Ok(None);
        }
        // Deserialize a map key.
        seed.deserialize(&mut *self).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        // Deserialize a map value.
        seed.deserialize(self)
    }
}
