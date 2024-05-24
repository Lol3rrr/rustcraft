#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub z: i32,
    pub y: i16,
}

impl Position {
    pub fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, super::ParseError> {
        let (i, raw) = nom::number::streaming::be_i64(i)?;

        let x = raw >> 38;
        let y = raw << 52 >> 52;
        let z = raw << 26 >> 38;

        Ok((
            i,
            Self {
                x: x as i32,
                z: z as i32,
                y: y as i16,
            },
        ))
    }
}

impl crate::serialize::SerializeItem for Position {
    fn slen(&self) -> usize {
        8
    }

    fn serialize<'b>(
        &self,
        buf: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        let mut value: i64 = 0;

        value |= (self.x as i64) << 38;
        value |= ((self.z as i64) << 12) & 0x0000003fffffffff;
        value |= (self.y as i64) & 0x0000000000000fff;

        value.serialize(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialize::SerializeItem;

    #[test]
    fn x0_y0_z0() {
        let (rem, res) =
            Position::parse(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]).unwrap();
        assert_eq!(&[0x01], rem);
        assert_eq!(Position { x: 0, z: 0, y: 0 }, res);

        let mut buffer = vec![0; 128];
        res.serialize(&mut buffer).unwrap();
        assert_eq!(
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            &buffer[..8]
        );
    }

    #[test]
    fn concrete_values() {
        let valid_bytes = &[
            0b01000110, 0b00000111, 0b01100011, 0b00101100, 0b00010101, 0b10110100, 0b10000011,
            0b00111111, 0x01,
        ];

        let (rem, res) = Position::parse(valid_bytes).unwrap();
        assert_eq!(&[0x01], rem);
        assert_eq!(
            Position {
                x: 18357644,
                z: -20882616,
                y: 831
            },
            res
        );

        let mut buffer = vec![0; 128];
        res.serialize(&mut buffer).unwrap();
        assert_eq!(&valid_bytes[..8], &buffer[..8]);
    }
}
