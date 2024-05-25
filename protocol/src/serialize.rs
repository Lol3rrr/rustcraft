#[derive(Debug)]
pub enum SerializeError {
    NotEnoughSpace { missing: usize },
    Other(&'static str),
}

pub trait SerializeItem {
    fn slen(&self) -> usize;
    fn serialize<'b>(&self, buf: &'b mut [u8]) -> Result<&'b mut [u8], SerializeError>;
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
}
