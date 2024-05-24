use crate::{EncryptedConnection, Transport};

pub struct UnencryptedConnection<S> {
    stream: S,
}

impl<S> UnencryptedConnection<S> {
    pub fn new(stream: S) -> Self {
        Self { stream }
    }

    pub fn encrypt(
        self,
        read: openssl::symm::Crypter,
        write: openssl::symm::Crypter,
    ) -> EncryptedConnection<S> {
        EncryptedConnection::new(self.stream, read, write)
    }
}

impl<S> Transport for UnencryptedConnection<S>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + core::marker::Unpin,
{
    async fn recv(&mut self, buf: &mut bytes::BytesMut) -> Result<usize, ()> {
        use tokio::io::AsyncReadExt;

        self.stream.read_buf(buf).await.map_err(|e| {
            dbg!(e);
            ()
        })
    }

    async fn send_packet<D>(&mut self, packet: &protocol::packet::Packet<D>) -> Result<(), ()>
    where
        D: protocol::packet::PacketContent,
    {
        use tokio::io::AsyncWriteExt;

        let bytes = packet.serialize();

        self.stream
            .write_all(&bytes)
            .await
            .map(|_| ())
            .map_err(|e| {
                dbg!(e);
                ()
            })
    }
}
