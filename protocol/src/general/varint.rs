#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct VarInt(pub i32);

impl VarInt {
    pub fn parse(mut i: &[u8]) -> nom::IResult<&[u8], Self, super::ParseError> {
        if i.is_empty() {
            return Err(nom::Err::Incomplete(nom::Needed::Unknown));
        }

        let mut value = 0;
        let mut idx = 0;
        while let Some(f_byte) = i.first() {
            i = &i[1..];

            value |= ((f_byte & 0x7f) as i32) << 7 * idx;
            idx += 1;

            if f_byte & 0x80 == 0 {
                return Ok((i, Self(value)));
            }
        }

        Err(nom::Err::Incomplete(nom::Needed::Unknown))
    }
}

impl crate::serialize::SerializeItem for VarInt {
    fn slen(&self) -> usize {
        5
    }

    fn serialize<'b>(
        &self,
        buf: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        (&mut buf[..5]).copy_from_slice(&[0x80, 0x80, 0x80, 0x80, 0x00]);

        let value = self.0 as u32;
        for (idx, cell) in buf.iter_mut().enumerate().take(5) {
            *cell |= ((value >> (idx * 7)) & 0x7f) as u8;
        }

        Ok(&mut buf[5..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialize::SerializeItem;

    #[test]
    fn parse_empty() {
        let err = VarInt::parse(&[]).unwrap_err();
        assert_eq!(nom::Err::Incomplete(nom::Needed::Unknown), err);
    }

    #[test]
    fn parse0() {
        let (rem, res) = VarInt::parse(&[0x00, 0x01]).unwrap();
        assert_eq!(&[0x01], rem);
        assert_eq!(VarInt(0), res);
    }

    #[test]
    fn parse127() {
        let (rem, res) = VarInt::parse(&[0x7f, 0x01]).unwrap();
        assert_eq!(&[0x01], rem);
        assert_eq!(VarInt(127), res);
    }

    #[test]
    fn parse128() {
        let (rem, res) = VarInt::parse(&[0x80, 0x01, 0x01]).unwrap();
        assert_eq!(&[0x01], rem);
        assert_eq!(VarInt(128), res);
    }

    #[test]
    fn parse_neg_1() {
        let (rem, res) = VarInt::parse(&[0xff, 0xff, 0xff, 0xff, 0x0f, 0x01]).unwrap();
        assert_eq!(&[0x01], rem);
        assert_eq!(VarInt(-1), res);
    }

    #[test]
    fn parse_neg_min() {
        let (rem, res) = VarInt::parse(&[0x80, 0x80, 0x80, 0x80, 0x08, 0x01]).unwrap();
        assert_eq!(&[0x01], rem);
        assert_eq!(VarInt(i32::MIN), res);
    }

    #[test]
    fn serialize_0() {
        let mut buffer = [0, 0, 0, 0, 0];
        VarInt(0).serialize(&mut buffer);
        assert_eq!([0x80, 0x80, 0x80, 0x80, 0x00], buffer);

        let (_, res) = VarInt::parse(&buffer).unwrap();
        assert_eq!(0, res.0);
    }

    #[test]
    fn serialize_9() {
        let mut buffer = [0, 0, 0, 0, 0];
        VarInt(9).serialize(&mut buffer);
        assert_eq!([0x89, 0x80, 0x80, 0x80, 0x00], buffer);

        let (_, res) = VarInt::parse(&buffer).unwrap();
        assert_eq!(9, res.0);
    }

    #[test]
    fn serialize_128() {
        let mut buffer = [0, 0, 0, 0, 0];
        VarInt(128).serialize(&mut buffer);
        assert_eq!([0x80, 0x81, 0x80, 0x80, 0x00], buffer);

        let (_, res) = VarInt::parse(&buffer).unwrap();
        assert_eq!(128, res.0);
    }

    #[test]
    fn serialize_neg_1() {
        let mut buffer = [0, 0, 0, 0, 0];
        VarInt(-1).serialize(&mut buffer);
        assert_eq!([0xff, 0xff, 0xff, 0xff, 0x0f], buffer);

        let (_, res) = VarInt::parse(&buffer).unwrap();
        assert_eq!(-1, res.0);
    }

    #[test]
    fn serialize_neg_max() {
        let mut buffer = [0, 0, 0, 0, 0];
        VarInt(i32::MIN).serialize(&mut buffer);
        assert_eq!([0x80, 0x80, 0x80, 0x80, 0x08], buffer);

        let (_, res) = VarInt::parse(&buffer).unwrap();
        assert_eq!(i32::MIN, res.0);
    }
}
