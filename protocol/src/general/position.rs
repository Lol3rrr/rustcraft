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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x0_y0_z0() {
        let (rem, res) =
            Position::parse(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]).unwrap();
        assert_eq!(&[0x01], rem);
        assert_eq!(Position { x: 0, z: 0, y: 0 }, res);
    }

    #[test]
    fn concrete_values() {
        let (rem, res) = Position::parse(&[
            0b01000110, 0b00000111, 0b01100011, 0b00101100, 0b00010101, 0b10110100, 0b10000011,
            0b00111111, 0x01,
        ])
        .unwrap();
        assert_eq!(&[0x01], rem);
        assert_eq!(
            Position {
                x: 18357644,
                z: -20882616,
                y: 831
            },
            res
        );
    }
}
