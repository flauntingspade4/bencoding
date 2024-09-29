use num::{Num, PrimInt};
use std::fmt::Display;
use std::ops::Neg;

use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;

use crate::err::{DeBencodingError, Result};

pub struct Deserializer<'de> {
    // The data being deserialized
    input: &'de [u8],
    // The index of the next character to be read in input
    offset: usize,
}

impl<'de> Deserializer<'de> {
    // By convention, `Deserializer` constructors are named like `from_xyz`.
    // That way basic use cases are satisfied by something like
    // `serde_json::from_str(...)` while advanced use cases that require a
    // deserializer can make one with `serde_json::Deserializer::from_str(...)`.
    pub const fn from_str(input: &'de str) -> Self {
        Self {
            input: input.as_bytes(),
            offset: 0,
        }
    }

    pub fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer { input, offset: 0 }
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
    if deserializer.offset == deserializer.input.len() {
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

    if deserializer.offset == deserializer.input.len() {
        Ok(t)
    } else {
        Err(DeBencodingError::TrailingCharacters)
    }
}

impl<'de: 'a, 'a> Deserializer<'de> {
    /// Peeks at the next byte in the input without consuming it
    fn peek_byte(&self) -> Result<&u8> {
        self.input.get(self.offset).ok_or(DeBencodingError::Eof)
    }

    /// Returns and consumes the byte at the current offset
    fn read_byte(&'a mut self) -> Result<u8> {
        let c = *self.peek_byte()?;
        self.offset += 1;
        Ok(c)
    }

    /// Returns and consumes the first n bytes from the current offset
    fn read_bytes(&mut self, len: usize) -> Result<&'a [u8]> {
        let bytes = &self
            .input
            .get(self.offset..self.offset + len)
            .ok_or(DeBencodingError::Eof)?;

        self.offset += len;

        Ok(bytes)
    }

    fn read_string(&mut self, len: usize) -> Result<&'a str> {
        std::str::from_utf8(self.read_bytes(len)?).map_err(|_| DeBencodingError::InputNotUtf8)
    }

    /// Read bytes from the input until it reaches a non-numeric ascii byte, then
    /// parses the read bytes into the given integer type and updates the offset
    fn read_integer<T>(&mut self) -> Result<T>
    where
        T: PrimInt + Display,
    {
        // Find the first non ascii-numeric byte
        let end_index = self
            .position_next(|&c| !('0'..'9').contains(&(c as char)))
            .ok_or(DeBencodingError::NoFoundClosingDeliminator)?;

        let ascii_string = std::str::from_utf8(&self.input[self.offset..end_index])
            .expect("Trying to read integer that is not valid ascii");

        let result = <T as Num>::from_str_radix(ascii_string, 10)
            .map_err(|_| DeBencodingError::ParseIntError);

        self.offset = end_index;

        result
    }

    fn parse_unsigned<T>(&mut self) -> Result<T>
    where
        T: PrimInt + Display,
    {
        if self.read_byte()? != b'i' {
            return Err(DeBencodingError::NoFoundOpeningDeliminator);
        }

        let result = self.read_integer::<T>();

        if self.read_byte()? == b'e' {
            result
        } else {
            Err(DeBencodingError::NoFoundClosingDeliminator)
        }
    }

    // Parse a possible minus sign followed by a group of decimal digits as a
    // signed integer of type T.
    fn parse_signed<T>(&mut self) -> Result<T>
    where
        T: PrimInt + Neg<Output = T> + Display,
    {
        if self.read_byte()? != b'i' {
            return Err(DeBencodingError::NoFoundOpeningDeliminator);
        }

        let result = if *self.peek_byte()? == b'-' {
            self.read_byte()?;
            -self.read_integer::<T>()?
        } else {
            self.read_integer::<T>()?
        };

        if self.read_byte()? == b'e' {
            Ok(result)
        } else {
            Err(DeBencodingError::NoFoundClosingDeliminator)
        }
    }

    fn parse_bytes(&mut self) -> Result<&'a [u8]> {
        let bytes_len = self.read_integer::<usize>()?;

        if self.read_byte()? != b':' {
            return Err(DeBencodingError::NoFoundColon);
        }

        self.read_bytes(bytes_len)
    }

    fn parse_str(&mut self) -> Result<&'a str> {
        let str_len = self.read_integer::<usize>()?;

        if self.read_byte()? != b':' {
            return Err(DeBencodingError::NoFoundColon);
        }

        self.read_string(str_len)
    }

    fn position_next<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(&u8) -> bool,
    {
        let index = self.input[self.offset..].iter().position(predicate)?;

        Some(self.offset + index)
    }
}

impl<'de: 'a, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = DeBencodingError;

    // Look at the input data to decide what Serde data model type to
    // deserialize as. Not all data formats are able to support this operation.
    // Formats that support `deserialize_any` are known as self-describing.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match *self.peek_byte()? as char {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                self.deserialize_str(visitor)
            }
            'i' => self.deserialize_i64(visitor),
            'l' => self.deserialize_seq(visitor),
            'd' => self.deserialize_map(visitor),
            c => Err(DeBencodingError::UnexpectedCharType(c)),
        }
    }

    fn deserialize_bool<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(DeBencodingError::UnexpectedCharType('b'))
    }

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
        visitor.visit_borrowed_str(self.parse_str()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(self.parse_bytes()?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(self.parse_bytes()?)
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
        if self.read_byte()? == b'l' {
            // Give the visitor access to each element of the sequence.
            let value = visitor.visit_seq(&mut self)?;
            // Parse the closing character of the sequence.
            if self.read_byte()? == b'e' {
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
        if self.read_byte()? == b'd' {
            // Give the visitor access to each entry of the map.
            let value = visitor.visit_map(&mut self)?;
            // Parse the closing brace of the map.
            if self.read_byte()? == b'e' {
                Ok(value)
            } else {
                Err(DeBencodingError::NoFoundClosingDeliminator)
            }
        } else {
            Err(DeBencodingError::NoFoundOpeningDeliminator)
        }
    }

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
        if self.peek_byte()? == &b'e' {
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
        if self.peek_byte()? == &b'e' {
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
