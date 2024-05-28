#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct VarLong(pub i64);

impl crate::serialize::SerializeItem for VarLong {
    fn slen(&self) -> usize {
        10
    }

    fn serialize<'b>(
        &self,
        buf: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        (&mut buf[..10])
            .copy_from_slice(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00]);

        let value = self.0 as u32;
        for (idx, cell) in buf.iter_mut().enumerate().take(10) {
            *cell |= ((value >> (idx * 7)) & 0x7f) as u8;
        }

        Ok(&mut buf[10..])
    }

    fn parse(mut i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        if i.is_empty() {
            return Err(nom::Err::Incomplete(nom::Needed::Unknown));
        }

        let mut value = 0;
        let mut idx = 0;
        while let Some(f_byte) = i.first() {
            i = &i[1..];

            value |= ((f_byte & 0x7f) as i64) << 7 * idx;
            idx += 1;

            if f_byte & 0x80 == 0 {
                return Ok((i, Self(value)));
            }
        }

        Err(nom::Err::Incomplete(nom::Needed::Unknown))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {}
}
