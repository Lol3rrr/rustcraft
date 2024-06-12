#[derive(Debug)]
pub enum SerializeError {
    NotEnoughSpace { missing: usize },
    Other(&'static str),
}

pub trait SerializeItem: Sized {
    fn slen(&self) -> usize;
    fn serialize<'b>(&self, buf: &'b mut [u8]) -> Result<&'b mut [u8], SerializeError>;

    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError>;
}

impl SerializeItem for bool {
    fn slen(&self) -> usize {
        1
    }
    fn serialize<'b>(&self, buf: &'b mut [u8]) -> Result<&'b mut [u8], SerializeError> {
        if buf.is_empty() {
            return Err(SerializeError::NotEnoughSpace { missing: 1 });
        }
        buf[0] = if *self { 1 } else { 0 };
        Ok(&mut buf[1..])
    }
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        let (i, raw) = nom::number::streaming::be_u8(i)?;
        Ok((i, raw == 0x01))
    }
}
impl SerializeItem for u8 {
    fn slen(&self) -> usize {
        1
    }
    fn serialize<'b>(&self, buf: &'b mut [u8]) -> Result<&'b mut [u8], SerializeError> {
        if buf.is_empty() {
            return Err(SerializeError::NotEnoughSpace { missing: 1 });
        }
        buf[0] = *self;
        Ok(&mut buf[1..])
    }
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        nom::number::streaming::be_u8(i)
    }
}
impl SerializeItem for i8 {
    fn slen(&self) -> usize {
        1
    }
    fn serialize<'b>(&self, buf: &'b mut [u8]) -> Result<&'b mut [u8], SerializeError> {
        if buf.is_empty() {
            return Err(SerializeError::NotEnoughSpace { missing: 1 });
        }
        (buf[..1]).copy_from_slice(&self.to_be_bytes());
        Ok(&mut buf[1..])
    }
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        nom::number::streaming::be_i8(i)
    }
}
impl SerializeItem for u16 {
    fn slen(&self) -> usize {
        2
    }
    fn serialize<'b>(&self, buf: &'b mut [u8]) -> Result<&'b mut [u8], SerializeError> {
        if buf.len() < 2 {
            return Err(SerializeError::NotEnoughSpace {
                missing: 2 - buf.len(),
            });
        }
        (buf[..2]).copy_from_slice(&self.to_be_bytes());
        Ok(&mut buf[2..])
    }
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        nom::number::streaming::be_u16(i)
    }
}
impl SerializeItem for i16 {
    fn slen(&self) -> usize {
        2
    }
    fn serialize<'b>(&self, buf: &'b mut [u8]) -> Result<&'b mut [u8], SerializeError> {
        if buf.len() < 2 {
            return Err(SerializeError::NotEnoughSpace {
                missing: 2 - buf.len(),
            });
        }
        (buf[..2]).copy_from_slice(&self.to_be_bytes());
        Ok(&mut buf[2..])
    }
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        nom::number::streaming::be_i16(i)
    }
}
impl SerializeItem for i32 {
    fn slen(&self) -> usize {
        4
    }
    fn serialize<'b>(&self, buf: &'b mut [u8]) -> Result<&'b mut [u8], SerializeError> {
        if buf.len() < 4 {
            return Err(SerializeError::NotEnoughSpace {
                missing: 4 - buf.len(),
            });
        }
        (buf[..4]).copy_from_slice(&self.to_be_bytes());
        Ok(&mut buf[4..])
    }
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        nom::number::streaming::be_i32(i)
    }
}
impl SerializeItem for i64 {
    fn slen(&self) -> usize {
        8
    }
    fn serialize<'b>(&self, buf: &'b mut [u8]) -> Result<&'b mut [u8], SerializeError> {
        if buf.len() < 8 {
            return Err(SerializeError::NotEnoughSpace {
                missing: 8 - buf.len(),
            });
        }
        (buf[..8]).copy_from_slice(&self.to_be_bytes());
        Ok(&mut buf[8..])
    }
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        nom::number::streaming::be_i64(i)
    }
}
impl SerializeItem for u128 {
    fn slen(&self) -> usize {
        16
    }
    fn serialize<'b>(&self, buf: &'b mut [u8]) -> Result<&'b mut [u8], SerializeError> {
        if buf.len() < 16 {
            return Err(SerializeError::NotEnoughSpace {
                missing: 16 - buf.len(),
            });
        }
        (buf[..16]).copy_from_slice(&self.to_be_bytes());
        Ok(&mut buf[16..])
    }
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        nom::number::streaming::be_u128(i)
    }
}
impl SerializeItem for f32 {
    fn slen(&self) -> usize {
        4
    }
    fn serialize<'b>(&self, buf: &'b mut [u8]) -> Result<&'b mut [u8], SerializeError> {
        if buf.len() < 4 {
            return Err(SerializeError::NotEnoughSpace {
                missing: 4 - buf.len(),
            });
        }
        (buf[..4]).copy_from_slice(&self.to_be_bytes());
        Ok(&mut buf[4..])
    }
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        nom::number::streaming::be_f32(i)
    }
}
impl SerializeItem for f64 {
    fn slen(&self) -> usize {
        8
    }
    fn serialize<'b>(&self, buf: &'b mut [u8]) -> Result<&'b mut [u8], SerializeError> {
        if buf.len() < 8 {
            return Err(SerializeError::NotEnoughSpace {
                missing: 8 - buf.len(),
            });
        }
        (buf[..8]).copy_from_slice(&self.to_be_bytes());
        Ok(&mut buf[8..])
    }
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        nom::number::streaming::be_f64(i)
    }
}

impl<T> SerializeItem for Option<T>
where
    T: SerializeItem,
{
    fn slen(&self) -> usize {
        1 + self.as_ref().map(|v| v.slen()).unwrap_or(0)
    }

    fn serialize<'b>(&self, mut buf: &'b mut [u8]) -> Result<&'b mut [u8], SerializeError> {
        match self.as_ref() {
            Some(v) => {
                buf = true.serialize(buf)?;
                v.serialize(buf)
            }
            None => false.serialize(buf),
        }
    }
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        let (i, exists) = bool::parse(i)?;
        if exists {
            let (i, value) = T::parse(i)?;
            Ok((i, Some(value)))
        } else {
            Ok((i, None))
        }
    }
}
impl<T> SerializeItem for Vec<T>
where
    T: SerializeItem,
{
    fn slen(&self) -> usize {
        crate::general::VarInt(self.len() as i32).slen()
            + self.iter().map(|i| i.slen()).sum::<usize>()
    }

    fn serialize<'b>(&self, mut buf: &'b mut [u8]) -> Result<&'b mut [u8], SerializeError> {
        buf = crate::general::VarInt(self.len() as i32).serialize(buf)?;
        for item in self.iter() {
            buf = item.serialize(buf)?;
        }
        Ok(buf)
    }

    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        let (mut i, raw_len) = crate::general::VarInt::parse(i)?;
        let mut parts = Vec::with_capacity(raw_len.0 as usize);

        for _ in 0..raw_len.0 {
            let (n_i, tmp) = T::parse(i)?;
            i = n_i;
            parts.push(tmp);
        }

        Ok((i, parts))
    }
}

impl<T1, T2> SerializeItem for (T1, T2)
where
    T1: SerializeItem,
    T2: SerializeItem,
{
    fn slen(&self) -> usize {
        self.0.slen() + self.1.slen()
    }

    fn serialize<'b>(&self, mut buf: &'b mut [u8]) -> Result<&'b mut [u8], SerializeError> {
        buf = self.0.serialize(buf)?;
        self.1.serialize(buf)
    }

    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        let (i, v1) = T1::parse(i)?;
        let (i, v2) = T2::parse(i)?;

        Ok((i, (v1, v2)))
    }
}

impl<T1, T2, T3> SerializeItem for (T1, T2, T3)
where
    T1: SerializeItem,
    T2: SerializeItem,
    T3: SerializeItem,
{
    fn slen(&self) -> usize {
        self.0.slen() + self.1.slen() + self.2.slen()
    }

    fn serialize<'b>(&self, mut buf: &'b mut [u8]) -> Result<&'b mut [u8], SerializeError> {
        buf = self.0.serialize(buf)?;
        buf = self.1.serialize(buf)?;
        buf = self.2.serialize(buf)?;
        Ok(buf)
    }

    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        let (i, v1) = T1::parse(i)?;
        let (i, v2) = T2::parse(i)?;
        let (i, v3) = T3::parse(i)?;

        Ok((i, (v1, v2, v3)))
    }
}

impl<T1, T2, T3, T4> SerializeItem for (T1, T2, T3, T4)
where
    T1: SerializeItem,
    T2: SerializeItem,
    T3: SerializeItem,
    T4: SerializeItem,
{
    fn slen(&self) -> usize {
        self.0.slen() + self.1.slen() + self.2.slen() + self.3.slen()
    }

    fn serialize<'b>(&self, mut buf: &'b mut [u8]) -> Result<&'b mut [u8], SerializeError> {
        buf = self.0.serialize(buf)?;
        buf = self.1.serialize(buf)?;
        buf = self.2.serialize(buf)?;
        buf = self.3.serialize(buf)?;
        Ok(buf)
    }

    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        let (i, v1) = T1::parse(i)?;
        let (i, v2) = T2::parse(i)?;
        let (i, v3) = T3::parse(i)?;
        let (i, v4) = T4::parse(i)?;

        Ok((i, (v1, v2, v3, v4)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i32_identity() {
        let mut tmp = [0, 0, 0, 0];

        let _ = 123_i32.serialize(&mut tmp).unwrap();
        assert_eq!((&[] as &[u8], 123), i32::parse(&tmp).unwrap());
    }
}
