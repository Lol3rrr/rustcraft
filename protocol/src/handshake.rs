//! All the Messages relating to the `Handshake` state of a connection

/// Clientbound messages
pub mod client {}

/// Serverbound messages
pub mod server {
    use crate::{
        general::{PString, VarInt},
        serialize::SerializeItem,
    };

    #[derive(Debug, PartialEq)]
    pub struct Handshaking {
        pub protocol_version: VarInt,
        pub server_addr: PString<'static>,
        pub server_port: u16,
        pub next_state: NextState,
    }

    /// Indicates the state that the connection should transition into after receiving/handling
    /// the [`Handshaking`] Packet
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum NextState {
        /// Moves into the `Status` state, corresponding messages can be found at [`crate::status`]
        Status,
        /// Moves into the `Login` state, corresponding messages can be found at [`crate::login`]
        Login,
    }

    impl Handshaking {
        pub fn parse<'i>(
            id: VarInt,
            i: &'i [u8],
        ) -> nom::IResult<&'i [u8], Self, crate::general::ParseError> {
            if id.0 != 0x00 {
                return Err(nom::Err::Error(crate::general::ParseError::Other));
            }

            let (i, prot_version) = VarInt::parse(i)?;
            let (i, server_addr) = PString::parse(i)?;
            let (i, server_port) = nom::number::streaming::be_u16(i)?;
            let (i, raw_next_state) = VarInt::parse(i)?;

            let next_state = match raw_next_state.0 {
                1 => NextState::Status,
                2 => NextState::Login,
                _ => return Err(nom::Err::Error(crate::general::ParseError::Other)),
            };

            Ok((
                i,
                Handshaking {
                    protocol_version: prot_version,
                    server_addr,
                    server_port,
                    next_state,
                },
            ))
        }
    }

    impl crate::packet::PacketContent for Handshaking {
        const ID: i32 = 0x00;
        const PACKETTRAIL: bool = false;

        fn length(&self) -> usize {
            self.protocol_version.slen()
                + self.server_addr.slen()
                + self.server_port.slen()
                + VarInt(0).slen()
        }

        fn serialize<'b>(
            &self,
            mut buffer: &'b mut [u8],
        ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
            buffer = self.protocol_version.serialize(buffer)?;
            buffer = self.server_addr.serialize(buffer)?;
            buffer = self.server_port.serialize(buffer)?;
            buffer = match &self.next_state {
                NextState::Status => VarInt(1).serialize(buffer)?,
                NextState::Login => VarInt(2).serialize(buffer)?,
            };
            Ok(buffer)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn handshake_status() {
            let (rem, hs) =
                Handshaking::parse(VarInt(0x00), &[0x06, 0x01, b'a', 0xff, 0x00, 0x01, 0xff])
                    .unwrap();
            assert_eq!(&[0xff], rem);
            assert_eq!(
                Handshaking {
                    protocol_version: VarInt(0x06),
                    server_addr: PString("a".into()),
                    server_port: 0xff00,
                    next_state: NextState::Status
                },
                hs
            );
        }

        #[test]
        fn handshake_login() {
            let (rem, hs) =
                Handshaking::parse(VarInt(0x00), &[0x06, 0x01, b'a', 0xff, 0x00, 0x02, 0xff])
                    .unwrap();
            assert_eq!(&[0xff], rem);
            assert_eq!(
                Handshaking {
                    protocol_version: VarInt(0x06),
                    server_addr: PString("a".into()),
                    server_port: 0xff00,
                    next_state: NextState::Login
                },
                hs
            );
        }
    }
}
