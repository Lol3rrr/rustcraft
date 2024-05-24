mod encrypted;
pub use encrypted::EncryptedConnection;

mod unencrypted;
pub use unencrypted::UnencryptedConnection;

mod utils;
pub use utils::{recv_legacy_packet, recv_packet};

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

