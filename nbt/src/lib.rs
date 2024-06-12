use std::collections::HashMap;

mod tag;
pub use tag::Tag;

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

impl<T> NbtTag for Vec<T>
where
    T: NbtTag,
{
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
        let (i, (inner, _)) =
            nom::multi::many_till(Tag::parse(false, false), nom::bytes::complete::tag([0x00]))(i)?;
        let (i, _) = nom::bytes::streaming::tag([0x00])(i)?;

        Ok((i, inner.into_iter().collect()))
    }
}

pub struct IntArray(pub Vec<i32>);
impl NbtTag for IntArray {
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
