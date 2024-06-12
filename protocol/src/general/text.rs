#[derive(Debug, PartialEq)]
pub struct TextComponent {
    pub content: nbt::Tag,
}

impl crate::serialize::SerializeItem for TextComponent {
    fn slen(&self) -> usize {
        todo!()
    }

    fn serialize<'b>(&self, buf: &'b mut [u8]) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        todo!()
    }

    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        let (i, (_, tag)) = nbt::Tag::parse(false, true)(i).map_err(|e| nom::Err::Error(crate::general::ParseError::Other))?;

        Ok((i, Self {
            content: tag,
        }))
    }
}


