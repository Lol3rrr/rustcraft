use std::borrow::Cow;

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct PString<'s>(pub Cow<'s, str>);

impl PString<'static> {
    pub fn parse<'i>(i: &'i [u8]) -> nom::IResult<&'i [u8], Self, super::ParseError> {
        let (i, length) = super::VarInt::parse(i)?;
        if length.0 < 0 {
            return Err(nom::Err::Error(super::ParseError::NegativeLength));
        }

        let len = length.0 as usize;
        let content = &i[..len];

        let str_content = core::str::from_utf8(content)
            .map_err(|_| nom::Err::Error(super::ParseError::ParseString))?;

        Ok((&i[len..], PString(Cow::Owned(str_content.to_string()))))
    }

    pub fn serialize_length(&self) -> usize {
        crate::general::VarInt(self.0.len() as i32).serialize_length() + self.0.len()
    }

    pub fn serialize(&self, mut buffer: &mut [u8]) -> usize {
        let length_varint = crate::general::VarInt(self.0.len() as i32);

        let written = length_varint.serialize(buffer);
        buffer = &mut buffer[written..];

        (&mut buffer[..self.0.len()]).copy_from_slice(self.0.as_bytes());

        written + self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let (rem, res) = PString::parse(&[0x00, 0x01]).unwrap();
        assert_eq!(&[0x01], rem);
        assert_eq!(PString(Cow::Borrowed("")), res);
    }

    #[test]
    fn non_empty_string() {
        let (rem, res) = PString::parse(&[0x04, b'a', b'c', b'd', b'b', 0x01]).unwrap();
        assert_eq!(&[0x01], rem);
        assert_eq!(PString("acdb".into()), res);
    }
}
