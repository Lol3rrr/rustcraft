use crate::{TransportReceive, TransportSend};

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

impl<S> TransportReceive for EncryptedConnection<S>
where
    S: tokio::io::AsyncRead + core::marker::Unpin,
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
}

impl<S> TransportSend for EncryptedConnection<S>
where
    S: tokio::io::AsyncWrite + core::marker::Unpin,
{
    async fn send_packet<D>(&mut self, packet: &protocol::packet::Packet<D>) -> Result<(), ()>
    where
        D: protocol::packet::PacketContent,
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

    async fn send_rawpacket(&mut self, packet: &protocol::packet::RawPacket) -> Result<(), ()> {
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
