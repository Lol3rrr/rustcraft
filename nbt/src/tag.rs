use std::collections::HashMap;

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
    pub fn parse(
        from_file: bool,
        network: bool,
    ) -> impl FnMut(&[u8]) -> nom::IResult<&[u8], (String, Self), ()> {
        move |i| {
            nom::branch::alt((
                nom::combinator::map(
                    nom::sequence::tuple((nom::bytes::streaming::tag([0x00]),)),
                    |_| (String::new(), Self::End),
                ),
                nom::combinator::map(
                    nom::sequence::tuple((
                        nom::bytes::streaming::tag([0x01]),
                        Self::parse_name,
                        nom::number::streaming::i8,
                    )),
                    |(_, name, v)| (name, Self::Byte(v)),
                ),
                nom::combinator::map(
                    nom::sequence::tuple((
                        nom::bytes::streaming::tag([0x02]),
                        Self::parse_name,
                        nom::number::streaming::be_i16,
                    )),
                    |(_, name, v)| (name, Self::Short(v)),
                ),
                nom::combinator::map(
                    nom::sequence::tuple((
                        nom::bytes::streaming::tag([0x03]),
                        Self::parse_name,
                        nom::number::streaming::be_i32,
                    )),
                    |(_, name, v)| (name, Self::Int(v)),
                ),
                nom::combinator::map(
                    nom::sequence::tuple((
                        nom::bytes::streaming::tag([0x04]),
                        Self::parse_name,
                        nom::number::streaming::be_i64,
                    )),
                    |(_, name, v)| (name, Self::Long(v)),
                ),
                nom::combinator::map(
                    nom::sequence::tuple((
                        nom::bytes::streaming::tag([0x05]),
                        Self::parse_name,
                        nom::number::streaming::be_f32,
                    )),
                    |(_, name, v)| (name, Self::Float(v)),
                ),
                nom::combinator::map(
                    nom::sequence::tuple((
                        nom::bytes::streaming::tag([0x06]),
                        Self::parse_name,
                        nom::number::streaming::be_f64,
                    )),
                    |(_, name, v)| (name, Self::Double(v)),
                ),
                nom::combinator::map(
                    nom::sequence::tuple((
                        nom::bytes::streaming::tag([0x07]),
                        Self::parse_name,
                        Self::parse_byte_array,
                    )),
                    |(_, name, v)| (name, Self::ByteArray(v)),
                ),
                nom::combinator::map(
                    nom::sequence::tuple((
                        nom::bytes::streaming::tag([0x08]),
                        Self::parse_name,
                        Self::parse_string,
                    )),
                    |(_, name, string)| (name, Self::String_(string)),
                ),
                nom::combinator::map(
                    nom::sequence::tuple((
                        nom::bytes::streaming::tag([0x09]),
                        Self::parse_name,
                        Self::parse_list,
                    )),
                    |(_, name, v)| (name, Self::List(v)),
                ),
                nom::combinator::map(
                    nom::sequence::tuple((
                        nom::bytes::streaming::tag([0x0a]),
                        nom::combinator::cond(!network, Self::parse_name),
                        Self::parse_compound_inner,
                    )),
                    |(_, name, parts)| (name.unwrap_or(String::new()), Self::Compound(parts)),
                ),
                nom::combinator::map(
                    nom::sequence::tuple((
                        nom::bytes::streaming::tag([0x0b]),
                        Self::parse_name,
                        Self::parse_int_array,
                    )),
                    |(_, name, v)| (name, Self::IntArray(v)),
                ),
                nom::combinator::map(
                    nom::sequence::tuple((
                        nom::bytes::streaming::tag([0x0c]),
                        Self::parse_name,
                        Self::parse_long_array,
                    )),
                    |(_, name, v)| (name, Self::LongArray(v)),
                ),
            ))(i)
        }
    }

    fn parse_name(i: &[u8]) -> nom::IResult<&[u8], String, ()> {
        let (i, length) = nom::number::streaming::be_u16(i)?;

        let raw_name = &i[..length as usize];
        let i = &i[length as usize..];

        let name = core::str::from_utf8(raw_name).unwrap();

        Ok((i, name.to_string()))
    }

    fn parse_byte_array(i: &[u8]) -> nom::IResult<&[u8], Vec<u8>, ()> {
        let (i, length) = nom::number::streaming::be_i32(i)?;

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

        let (i, length) = nom::number::streaming::be_i32(i)?;

        let mut result = Vec::with_capacity(length as usize);
        for _ in 0..length {
            todo!("How do we only parse the content but not the name?")
        }

        Ok((i, result))
    }

    fn parse_compound_inner(i: &[u8]) -> nom::IResult<&[u8], HashMap<String, Tag>, ()> {
        let (i, (inner, _)) =
            nom::multi::many_till(Self::parse(false, false), nom::bytes::complete::tag([0x00]))(i)?;

        Ok((i, inner.into_iter().collect()))
    }

    fn parse_int_array(i: &[u8]) -> nom::IResult<&[u8], Vec<i32>, ()> {
        let (mut i, length) = nom::number::streaming::be_i32(i)?;

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

        let mut result = Vec::with_capacity(length as usize);
        for _ in 0..length {
            let (n_i, r) = nom::number::streaming::be_i64(i)?;
            i = n_i;
            result.push(r);
        }

        Ok((i, result))
    }

    pub fn serialize(&self, file: bool, network: bool, name: Option<&str>, buf: &mut Vec<u8>) -> Result<(), ()> {
        match self {
            Self::End => {
                buf.push(0x00);

                Ok(())
            }
            Self::Byte(val) => {
                buf.push(0x01);

                let name = name.ok_or(())?;
                buf.extend((name.len() as u16).to_be_bytes());
                buf.extend(name.as_bytes());

                buf.extend(val.to_be_bytes());

                Ok(())
            }
            Self::Short(val) => {
                buf.push(0x02);

                let name = name.ok_or(())?;
                buf.extend((name.len() as u16).to_be_bytes());
                buf.extend(name.as_bytes());

                buf.extend(val.to_be_bytes());

                Ok(())
            }
            Self::Int(val) => {
                buf.push(0x03);

                let name = name.ok_or(())?;
                buf.extend((name.len() as u16).to_be_bytes());
                buf.extend(name.as_bytes());

                buf.extend(val.to_be_bytes());

                Ok(())
            }
            Self::Long(val) => {
                buf.push(0x04);

                let name = name.ok_or(())?;
                buf.extend((name.len() as u16).to_be_bytes());
                buf.extend(name.as_bytes());

                buf.extend(val.to_be_bytes());

                Ok(())
            }
            Self::Float(val) => {
                buf.push(0x05);

                let name = name.ok_or(())?;
                buf.extend((name.len() as u16).to_be_bytes());
                buf.extend(name.as_bytes());

                buf.extend(val.to_be_bytes());

                Ok(())
            }
            Self::Double(val) => {
                buf.push(0x06);

                let name = name.ok_or(())?;
                buf.extend((name.len() as u16).to_be_bytes());
                buf.extend(name.as_bytes());

                buf.extend(val.to_be_bytes());

                Ok(())
            }
            Self::String_(val) => {
                buf.push(0x08);

                let name = name.ok_or(())?;
                buf.extend((name.len() as u16).to_be_bytes());
                buf.extend(name.as_bytes());

                buf.extend((val.len() as u16).to_be_bytes());
                buf.extend(val.as_bytes());

                Ok(())
            }
            Self::ByteArray(vs) => {
                buf.push(0x07);

                let name = name.ok_or(())?;
                buf.extend((name.len() as u16).to_be_bytes());
                buf.extend(name.as_bytes());

                buf.extend((vs.len() as i32).to_be_bytes());
                buf.extend(vs.iter().copied());

                Ok(())
            }
            Self::IntArray(vs) => {
                buf.push(0x0b);

                let name = name.ok_or(())?;
                buf.extend((name.len() as u16).to_be_bytes());
                buf.extend(name.as_bytes());

                buf.extend((vs.len() as i32).to_be_bytes());

                buf.extend(vs.iter().flat_map(|v| v.to_be_bytes()));

                Ok(())
            }
            Self::LongArray(vs) => {
                buf.push(0x0c);

                let name = name.ok_or(())?;
                buf.extend((name.len() as u16).to_be_bytes());
                buf.extend(name.as_bytes());

                buf.extend((vs.len() as i32).to_be_bytes());

                buf.extend(vs.iter().flat_map(|v| v.to_be_bytes()));

                Ok(())
            }
            Self::List(vs) => {
                dbg!(vs);
                todo!("List")
            }
            Self::Compound(vs) => {
                dbg!(vs);
                todo!("Compound")
            }
        }
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
        assert_eq!(
            (
                "hello world".into(),
                Tag::Compound(
                    [("name".into(), Tag::String_("Bananrama".into()))]
                        .into_iter()
                        .collect()
                )
            ),
            result
        );
    }
}

#[cfg(test)]
mod serialize_parse_tests {
    use super::*;

    macro_rules! parsed {
        ($value:expr, $name:literal) => {{
            let src_value = $value;

            let mut buffer = Vec::new();
            src_value.serialize(false, true, Some($name), &mut buffer).unwrap();

            let (rem, (parsed_name, parsed_value)) = Tag::parse(false, true)(&buffer).unwrap();
            assert_eq!(&[] as &[u8], rem);
            assert_eq!($name, parsed_name);
            assert_eq!(src_value, parsed_value);
        }}
    }

    #[test]
    fn byte() {
        parsed!(Tag::Byte(123), "test");
    }

    #[test]
    fn short() {
        parsed!(Tag::Short(234), "test");
    }

    #[test]
    fn int() {
        parsed!(Tag::Int(234), "test");
    }

    #[test]
    fn long() {
        parsed!(Tag::Long(234), "test");
    }

    #[test]
    fn float() {
        parsed!(Tag::Float(234.0), "test");
    }

    #[test]
    fn doulbe() {
        parsed!(Tag::Double(234.0), "test");
    }

    #[test]
    fn string() {
        parsed!(Tag::String_("other".into()), "test");
    }

    #[test]
    fn byte_array() {
        parsed!(Tag::ByteArray(vec![0, 1, 2, 128, 127]), "test");
    }

    #[test]
    fn list() {
        parsed!(Tag::List(vec![Tag::Float(123.0), Tag::Float(321.0)]), "test");
    }

    #[test]
    fn int_array() {
        parsed!(Tag::IntArray(vec![0, 1, 2, 128, 127]), "test");
    }

    #[test]
    fn long_array() {
        parsed!(Tag::LongArray(vec![0, 1, 2, 128, 127]), "test");
    }

    #[test]
    fn compound_float() {
        parsed!(Tag::Compound(vec![("first".to_string(), Tag::Float(123.0))].into_iter().collect()), "test");
    }
}
