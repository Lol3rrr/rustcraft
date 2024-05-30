use crate::general::VarInt;

#[derive(Debug, PartialEq)]
pub struct Slot {
    pub values: Option<(VarInt, i8, nbt::Tag)>,
}

impl crate::serialize::SerializeItem for Slot {
    fn slen(&self) -> usize {
        todo!()
    }

    fn serialize<'b>(
        &self,
        buf: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        todo!()
    }

    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        let (i, present) = bool::parse(i)?;

        if present {
            let (i, itemid) = VarInt::parse(i)?;
            let (i, count) = i8::parse(i)?;
            let (i, (_, data)) = nbt::Tag::parse(false, true)(i)
                .map_err(|e| nom::Err::Error(crate::general::ParseError::Other))?;

            Ok((
                i,
                Self {
                    values: Some((itemid, count, data)),
                },
            ))
        } else {
            Ok((i, Self { values: None }))
        }
    }
}
