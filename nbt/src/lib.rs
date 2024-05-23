use std::collections::HashMap;

pub trait NbtTag: Sized {
    const ID: u8;
    const WITHNAME: bool;

    fn parse_value(i: &[u8]) -> nom::IResult<&[u8], Self, ()>;
}

impl NbtTag for () {
    const ID: u8 = 0x00;
    const WITHNAME: bool = false;

    fn parse_value(i: &[u8]) -> nom::IResult<&[u8], Self, ()> {
        Ok((i, ()))
    }
}

impl NbtTag for i8 {
    const ID: u8 = 0x01;
    const WITHNAME: bool = true;

    fn parse_value(i: &[u8]) -> nom::IResult<&[u8], Self, ()> {
        nom::number::streaming::i8(i)
    }
}

impl NbtTag for i16 {
    const ID: u8 = 0x02;
    const WITHNAME: bool = true;

    fn parse_value(i: &[u8]) -> nom::IResult<&[u8], Self, ()> {
        nom::number::streaming::be_i16(i)
    }
}

impl NbtTag for i32 {
    const ID: u8 = 0x03;
    const WITHNAME: bool = true;

    fn parse_value(i: &[u8]) -> nom::IResult<&[u8], Self, ()> {
        nom::number::streaming::be_i32(i)
    }
}

impl NbtTag for i64 {
    const ID: u8 = 0x04;
    const WITHNAME: bool = true;

    fn parse_value(i: &[u8]) -> nom::IResult<&[u8], Self, ()> {
        nom::number::streaming::be_i64(i)
    }
}

impl NbtTag for f32 {
    const ID: u8 = 0x05;
    const WITHNAME: bool = true;

    fn parse_value(i: &[u8]) -> nom::IResult<&[u8], Self, ()> {
        nom::number::streaming::be_f32(i)
    }
}

impl NbtTag for f64 {
    const ID: u8 = 0x06;
    const WITHNAME: bool = true;

    fn parse_value(i: &[u8]) -> nom::IResult<&[u8], Self, ()> {
        nom::number::streaming::be_f64(i)
    }
}

pub struct ByteArray(pub Vec<u8>);
impl NbtTag for ByteArray {
    const ID: u8 = 0x07;
    const WITHNAME: bool = true;

    fn parse_value(i: &[u8]) -> nom::IResult<&[u8], Self, ()> {
        let (i, length) = nom::number::streaming::be_i32(i)?;

        let raw_content = &i[..length as usize];
        let i = &i[length as usize..];

        Ok((i, ByteArray(raw_content.to_vec())))
    }
}

impl NbtTag for String {
    const ID: u8 = 0x08;
    const WITHNAME: bool = true;

    fn parse_value(i: &[u8]) -> nom::IResult<&[u8], Self, ()> {
        let (i, length) = nom::number::streaming::be_u16(i)?;

        let raw_str = core::str::from_utf8(&i[..length as usize]).unwrap();
        let i = &i[length as usize..];

        Ok((i, raw_str.to_string()))
    }
}

impl<T> NbtTag for Vec<T> where T: NbtTag {
    const ID: u8 = 0x09;
    const WITHNAME: bool = true;

    fn parse_value(i: &[u8]) -> nom::IResult<&[u8], Self, ()> {
        let (i, expected_tag) = nom::number::streaming::u8(i)?;
        assert_eq!(expected_tag, T::ID);

        let (mut i, length) = nom::number::streaming::be_i32(i)?;

        let mut result = Vec::with_capacity(length as usize);
        for _ in 0..length {
            let (n_i, r) = T::parse_value(i)?;
            i = n_i;
            result.push(r);
        }

        Ok((i, result))
    }
}

impl NbtTag for HashMap<String, Tag> {
    const ID: u8 = 0x0a;
    const WITHNAME: bool = true;

    fn parse_value(i: &[u8]) -> nom::IResult<&[u8], Self, ()> {
        let (i, (inner, _)) = nom::multi::many_till(Tag::parse(false, false), nom::bytes::complete::tag([0x00]))(i)?;
        let (i, _) = nom::bytes::streaming::tag([0x00])(i)?;

        Ok((i, inner.into_iter().collect()))
    }
}

pub struct IntArray(pub Vec<i32>);
impl NbtTag for IntArray{
    const ID: u8 = 0x0b;
    const WITHNAME: bool = true;

    fn parse_value(i: &[u8]) -> nom::IResult<&[u8], Self, ()> {
        let (mut i, length) = nom::number::streaming::be_i32(i)?;

        let mut result = Vec::with_capacity(length as usize);
        for _ in 0..length {
            let (n_i, r) = nom::number::streaming::be_i32(i)?;
            i = n_i;
            result.push(r);
        }

        Ok((i, IntArray(result)))
    }
}

pub struct LongArray(pub Vec<i64>);
impl NbtTag for LongArray {
    const ID: u8 = 0x0c;
    const WITHNAME: bool = true;

    fn parse_value(i: &[u8]) -> nom::IResult<&[u8], Self, ()> {
        let (mut i, length) = nom::number::streaming::be_i32(i)?;

        let mut result = Vec::with_capacity(length as usize);
        for _ in 0..length {
            let (n_i, r) = nom::number::streaming::be_i64(i)?;
            i = n_i;
            result.push(r);
        }

        Ok((i, LongArray(result)))
    }
}

#[derive(Debug, PartialEq)]
pub enum Tag {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String_(String),
    List(Vec<Tag>),
    Compound(HashMap<String, Tag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Tag {
    pub fn parse(from_file: bool, network: bool) -> impl FnMut(&[u8]) -> nom::IResult<&[u8], (String, Self), ()> {
        move |i| {
            nom::branch::alt((
                nom::combinator::map(nom::sequence::tuple((
                    nom::bytes::streaming::tag([0x00]),
                )), |_| (String::new(), Self::End)),
                nom::combinator::map(nom::sequence::tuple((
                    nom::bytes::streaming::tag([0x01]),
                    Self::parse_name,
                    nom::number::streaming::i8,
                )), |(_, name, v)| (name, Self::Byte(v))),
                nom::combinator::map(nom::sequence::tuple((
                    nom::bytes::streaming::tag([0x02]),
                    Self::parse_name,
                    nom::number::streaming::be_i16,
                )), |(_, name, v)| (name, Self::Short(v))),
                nom::combinator::map(nom::sequence::tuple((
                    nom::bytes::streaming::tag([0x03]),
                    Self::parse_name,
                    nom::number::streaming::be_i32,
                )), |(_, name, v)| (name, Self::Int(v))),
                nom::combinator::map(nom::sequence::tuple((
                    nom::bytes::streaming::tag([0x04]),
                    Self::parse_name,
                    nom::number::streaming::be_i64,
                )), |(_, name, v)| (name, Self::Long(v))),
                nom::combinator::map(nom::sequence::tuple((
                    nom::bytes::streaming::tag([0x05]),
                    Self::parse_name,
                    nom::number::streaming::be_f32,
                )), |(_, name, v)| (name, Self::Float(v))),
                nom::combinator::map(nom::sequence::tuple((
                    nom::bytes::streaming::tag([0x06]),
                    Self::parse_name,
                    nom::number::streaming::be_f64,
                )), |(_, name, v)| (name, Self::Double(v))),
                nom::combinator::map(nom::sequence::tuple((
                    nom::bytes::streaming::tag([0x07]),
                    Self::parse_name,
                    Self::parse_byte_array,
                )), |(_, name, v)| (name, Self::ByteArray(v))),
                nom::combinator::map(nom::sequence::tuple((
                    nom::bytes::streaming::tag([0x08]),
                    Self::parse_name,
                    Self::parse_string,
                )), |(_, name, string)| (name, Self::String_(string))),
                nom::combinator::map(nom::sequence::tuple((
                    nom::bytes::streaming::tag([0x09]),
                    Self::parse_name,
                    Self::parse_list,
                )), |(_, name, v)| (name, Self::List(v))),
                nom::combinator::map(nom::sequence::tuple((
                    nom::bytes::streaming::tag([0x0a]),
                    Self::parse_name,
                    Self::parse_compound_inner,
                )), |(_, name, parts)| (name, Self::Compound(parts))),
                nom::combinator::map(nom::sequence::tuple((
                    nom::bytes::streaming::tag([0x0b]),
                    Self::parse_name,
                    Self::parse_int_array,
                )), |(_, name, v)| (name, Self::IntArray(v))),
                nom::combinator::map(nom::sequence::tuple((
                    nom::bytes::streaming::tag([0x0c]),
                    Self::parse_name,
                    Self::parse_long_array,
                )), |(_, name, v)| (name, Self::LongArray(v))),
            ))(i)
        }
    }
    
    fn parse_name(i: &[u8]) -> nom::IResult<&[u8], String, ()> {
        let (i, length) = nom::number::streaming::be_u16(i)?;
        dbg!(i, length);

        let raw_name = &i[..length as usize];
        let i = &i[length as usize..];

        let name = core::str::from_utf8(raw_name).unwrap();
        dbg!(name);

        Ok((i, name.to_string()))
    }

    fn parse_byte_array(i: &[u8]) -> nom::IResult<&[u8], Vec<u8>, ()> {
        let (i, length) = nom::number::streaming::be_i32(i)?;
        dbg!(i, length);

        let raw_content = &i[..length as usize];
        let i = &i[length as usize..];

        Ok((i, raw_content.to_vec()))
    }

    fn parse_string(i: &[u8]) -> nom::IResult<&[u8], String, ()> {
        let (i, length) = nom::number::streaming::be_u16(i)?;

        let raw_str = core::str::from_utf8(&i[..length as usize]).unwrap();
        let i = &i[length as usize..];

        Ok((i, raw_str.to_string()))
    }

    fn parse_list(i: &[u8]) -> nom::IResult<&[u8], Vec<Tag>, ()> {
        let (i, expected_tag) = nom::number::streaming::u8(i)?;
        dbg!(i, expected_tag);

        let (i, length) = nom::number::streaming::be_i32(i)?;
        dbg!(i, length);

        let mut result = Vec::with_capacity(length as usize);
        for _ in 0..length {
            todo!("How do we only parse the content but not the name?")
        }

        Ok((i, result))
    }

    fn parse_compound_inner(i: &[u8]) -> nom::IResult<&[u8], HashMap<String, Tag>, ()> {
        let (i, (inner, _)) = nom::multi::many_till(Self::parse(false, false), nom::bytes::complete::tag([0x00]))(i)?;

        dbg!(&inner);

        Ok((i, inner.into_iter().collect()))
    }

    fn parse_int_array(i: &[u8]) -> nom::IResult<&[u8], Vec<i32>, ()> {
        let (mut i, length) = nom::number::streaming::be_i32(i)?;
        dbg!(i, length);

        let mut result = Vec::with_capacity(length as usize);
        for _ in 0..length {
            let (n_i, r) = nom::number::streaming::be_i32(i)?;
            i = n_i;
            result.push(r);
        }

        Ok((i, result))
    }

    fn parse_long_array(i: &[u8]) -> nom::IResult<&[u8], Vec<i64>, ()> {
        let (mut i, length) = nom::number::streaming::be_i32(i)?;
        dbg!(i, length);

        let mut result = Vec::with_capacity(length as usize);
        for _ in 0..length {
            let (n_i, r) = nom::number::streaming::be_i64(i)?;
            i = n_i;
            result.push(r);
        }

        Ok((i, result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_hello_world() {
        let content = include_bytes!("../test-files/hello_world.nbt");

        let (rem, result) = Tag::parse(false, false)(content).unwrap();
        assert_eq!(&[] as &[u8], rem);
        assert_eq!(("hello world".into(), Tag::Compound([
            ("name".into(), Tag::String_("Bananrama".into()))
        ].into_iter().collect())), result);
    }
}
