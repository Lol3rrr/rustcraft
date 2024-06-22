use std::borrow::Cow;

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct PString<'s>(pub Cow<'s, str>);

impl<'s> crate::serialize::SerializeItem for PString<'s> {
    fn slen(&self) -> usize {
        crate::general::VarInt(self.0.len() as i32).slen() + self.0.len()
    }

    fn serialize<'b>(
        &self,
        mut buf: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        let length_varint = crate::general::VarInt(self.0.len() as i32);

        buf = length_varint.serialize(buf)?;

        (&mut buf[..self.0.len()]).copy_from_slice(self.0.as_bytes());
        Ok(&mut buf[self.0.len()..])
    }

    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        let (i, length) = super::VarInt::parse(i)?;
        if length.0 < 0 {
            return Err(nom::Err::Error(super::ParseError::NegativeLength));
        }

        let len = length.0 as usize;
        if i.len() < len {
            dbg!(i);
            dbg!(len);
            return Err(nom::Err::Error(super::ParseError::Other));
        }

        let content = &i[..len];

        let str_content = core::str::from_utf8(content)
            .map_err(|_| nom::Err::Error(super::ParseError::ParseString))?;

        Ok((&i[len..], PString(Cow::Owned(str_content.to_string()))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialize::SerializeItem;

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

    #[test]
    fn serialize_empty() {
        let mut buffer = vec![0; 10];

        let rem = PString("test".into()).serialize(&mut buffer).unwrap();
        assert_eq!(5, rem.len(), "{:?}", rem);

        let (unparsed, parsed) = PString::parse(&buffer).unwrap();
    }
}
