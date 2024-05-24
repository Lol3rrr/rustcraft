use crate::Connection;

pub async fn recv_packet<S, D, F>(
    s: &mut S,
    buffer: &mut bytes::BytesMut,
    mut parser: F,
) -> Result<protocol::packet::Packet<D>, ()>
where
    F: protocol::packet::PacketContentParser<D>,
    S: Connection,
{
    loop {
        match protocol::packet::Packet::parse_bytes(&mut parser, buffer) {
            Ok((_, v)) => return Ok(v),
            Err(nom::Err::Incomplete(_)) => {}
            Err(e) => {
                tracing::error!(?e);
                dbg!(e);
                return Err(());
            }
        };

        tracing::debug!("Incomplete Data");

        match s.recv(buffer).await {
            Ok(v) if v == 0 => {
                tracing::error!("EOF");
                return Err(());
            }
            Err(e) => {
                tracing::error!(?e);
                dbg!(e);
                return Err(());
            }
            _ => {}
        };
    }
}

pub async fn recv_legacy_packet<S, D, F>(
    s: &mut S,
    buffer: &mut bytes::BytesMut,
    mut parser: F,
) -> Result<protocol::packet::LegacyPacket<D>, ()>
where
    F: protocol::packet::PacketContentParser<D>,
    S: Connection,
{
    loop {
        match protocol::packet::LegacyPacket::parse_bytes(&mut parser, buffer) {
            Ok((_, v)) => return Ok(v),
            Err(nom::Err::Incomplete(_)) => {}
            Err(e) => {
                tracing::error!(?e);
                dbg!(e);
                return Err(());
            }
        };

        tracing::debug!("Incomplete Data");

        match s.recv(buffer).await {
            Ok(v) if v == 0 => {
                tracing::error!("EOF");
                return Err(());
            }
            Err(e) => {
                tracing::error!(?e);
                dbg!(e);
                return Err(());
            }
            _ => {}
        };
    }
}
