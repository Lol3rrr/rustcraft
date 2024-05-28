#[derive(Debug, PartialEq)]
pub struct BitSet {
    parts: Vec<i64>,
}

impl crate::serialize::SerializeItem for BitSet {
    fn slen(&self) -> usize {
        self.parts.slen()
    }

    fn serialize<'b>(
        &self,
        mut buffer: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        buffer = self.parts.serialize(buffer)?;

        Ok(buffer)
    }

    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        let (i, parts) = Vec::<i64>::parse(i)?;
        Ok((i, Self { parts }))
    }
}

impl BitSet {
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }
}
