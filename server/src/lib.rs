use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProfileResponse {
    pub id: String,
    pub name: String,
    pub properties: Vec<ProfileProperty>,
    pub profileActions: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: String,
}

pub struct UnencryptedConnection<S> {
    stream: S,
}
pub struct EncryptedConnection<S> {
    stream: S,
    read_c: openssl::symm::Crypter,
    write_c: openssl::symm::Crypter,
    recv_target: Vec<u8>,
    decrypt_target: Vec<u8>,
}

impl<S> EncryptedConnection<S> {
    pub fn new(stream: S, read: openssl::symm::Crypter, write: openssl::symm::Crypter) -> Self {
        Self {
            stream,
            read_c: read,
            write_c: write,
            recv_target: vec![0; 1028],
            decrypt_target: vec![0; 1028],
        }
    }
}

impl<S> Connection for EncryptedConnection<S>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + core::marker::Unpin,
{
    async fn recv(&mut self, buf: &mut bytes::BytesMut) -> Result<usize, ()> {
        use bytes::BufMut;
        use tokio::io::AsyncReadExt;

        let read = self.stream.read(&mut self.recv_target).await.map_err(|e| {
            dbg!(e);
            ()
        })?;
        if read == 0 {
            return Ok(0);
        }

        let decrypted = self
            .read_c
            .update(&self.recv_target[..read], &mut self.decrypt_target)
            .map_err(|e| {
                dbg!(e);
                ()
            })?;
        if decrypted == 0 {
            todo!();
        }

        buf.put_slice(&self.decrypt_target[..decrypted]);

        Ok(decrypted)
    }

    async fn send_packet<D>(&mut self, packet: &protocol::packet::Packet<D>) -> Result<(), ()>
    where
        D: protocol::packet::PacketContentSerializer,
    {
        use tokio::io::AsyncWriteExt;

        let raw_bytes = packet.serialize();
        let mut output = vec![0; raw_bytes.len()];

        let encrypted = self.write_c.update(&raw_bytes, &mut output).unwrap();

        self.stream
            .write_all(&output[..encrypted])
            .await
            .map(|_| ())
            .map_err(|e| {
                dbg!(e);
                ()
            })
    }
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

impl<S> Connection for UnencryptedConnection<S>
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
        D: protocol::packet::PacketContentSerializer,
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

pub trait Connection {
    fn recv(
        &mut self,
        buf: &mut bytes::BytesMut,
    ) -> impl core::future::Future<Output = Result<usize, ()>>;

    fn send_packet<D>(
        &mut self,
        packet: &protocol::packet::Packet<D>,
    ) -> impl core::future::Future<Output = Result<(), ()>>
    where
        D: protocol::packet::PacketContentSerializer;
}

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
