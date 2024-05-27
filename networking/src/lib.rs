mod encrypted;
pub use encrypted::EncryptedConnection;

mod unencrypted;
pub use unencrypted::UnencryptedConnection;

pub trait Transport: TransportReceive + TransportSend {}
impl<T> Transport for T where T: TransportSend + TransportReceive {}

pub trait TransportReceive {
    fn recv(
        &mut self,
        buf: &mut bytes::BytesMut,
    ) -> impl core::future::Future<Output = Result<usize, ()>>;
}
pub trait TransportSend {
    fn send_packet<D>(
        &mut self,
        packet: &protocol::packet::Packet<D>,
    ) -> impl core::future::Future<Output = Result<(), ()>>
    where
        D: protocol::packet::PacketContent;

    fn send_rawpacket(
        &mut self,
        packet: &protocol::packet::RawPacket,
    ) -> impl core::future::Future<Output = Result<(), ()>>;
}

pub struct Connection<T> {
    transport: T,
    buffer: bytes::BytesMut,
}

impl<T> Connection<T> {
    pub fn new(transport: T, buffer: bytes::BytesMut) -> Self {
        Self { transport, buffer }
    }

    pub fn map_transport<F, T2>(self, func: F) -> Connection<T2>
    where
        F: FnOnce(T) -> T2,
    {
        let n_transport = func(self.transport);

        Connection {
            transport: n_transport,
            buffer: self.buffer,
        }
    }
}
impl<T> Connection<T>
where
    T: TransportReceive,
{
    pub async fn recv_rawpacket(&mut self) -> Result<protocol::packet::RawPacket, ()> {
        loop {
            match protocol::packet::RawPacket::parse_bytes(&mut self.buffer) {
                Ok((_, v)) => return Ok(v),
                Err(nom::Err::Incomplete(_)) => {}
                Err(e) => {
                    tracing::error!(?e);
                    return Err(());
                }
            };

            match self.transport.recv(&mut self.buffer).await {
                Ok(v) if v == 0 => {
                    tracing::error!("EOF");
                    return Err(());
                }
                Err(e) => {
                    tracing::error!(?e);
                    return Err(());
                }
                _ => {}
            };
        }
    }

    pub async fn recv_packet<D, F>(
        &mut self,
        mut parser: F,
    ) -> Result<protocol::packet::Packet<D>, ()>
    where
        F: protocol::packet::PacketContentParser<D>,
    {
        loop {
            match protocol::packet::Packet::parse_bytes(&mut parser, &mut self.buffer) {
                Ok((_, v)) => return Ok(v),
                Err(nom::Err::Incomplete(_)) => {}
                Err(e) => {
                    tracing::error!(?e);
                    return Err(());
                }
            };

            match self.transport.recv(&mut self.buffer).await {
                Ok(v) if v == 0 => {
                    tracing::error!("EOF");
                    return Err(());
                }
                Err(e) => {
                    tracing::error!(?e);
                    return Err(());
                }
                _ => {}
            };
        }
    }

    pub async fn recv_legacy_packet<D, F>(
        &mut self,
        mut parser: F,
    ) -> Result<protocol::packet::LegacyPacket<D>, ()>
    where
        F: protocol::packet::PacketContentParser<D>,
    {
        loop {
            match protocol::packet::LegacyPacket::parse_bytes(&mut parser, &mut self.buffer) {
                Ok((_, v)) => return Ok(v),
                Err(nom::Err::Incomplete(_)) => {}
                Err(e) => {
                    tracing::error!(?e);
                    return Err(());
                }
            };

            match self.transport.recv(&mut self.buffer).await {
                Ok(v) if v == 0 => {
                    tracing::error!("EOF");
                    return Err(());
                }
                Err(e) => {
                    tracing::error!(?e);
                    return Err(());
                }
                _ => {}
            };
        }
    }
}

impl<T> Connection<T>
where
    T: TransportSend,
{
    pub async fn send_packet<D>(&mut self, packet: &protocol::packet::Packet<D>) -> Result<(), ()>
    where
        D: protocol::packet::PacketContent,
    {
        self.transport.send_packet(packet).await
    }

    pub async fn send_rawpacket(&mut self, packet: &protocol::packet::RawPacket) -> Result<(), ()> {
        self.transport.send_rawpacket(packet).await
    }
}
