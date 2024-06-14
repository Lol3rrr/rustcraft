use tracing_subscriber::layer::SubscriberExt;

use networking::{Connection, Transport};

fn main() {
    let fmt_layer = tracing_subscriber::fmt::layer().with_ansi(false);
    tracing::subscriber::set_global_default(
        tracing_subscriber::Registry::default().with(fmt_layer),
    )
    .unwrap();

    tracing::info!("Starting");

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(run_server());
}

#[tracing::instrument]
async fn run_server() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:25565")
        .await
        .unwrap();

    tracing::info!("Waiting for connections");

    while let Ok((connection, addr)) = listener.accept().await {
        tracing::info!(?addr);

        let target_connection = tokio::net::TcpStream::connect("127.0.0.1:35565")
            .await
            .unwrap();

        tokio::spawn(handle_connection(connection, target_connection));
    }

    tracing::error!("Stopped");
}

#[tracing::instrument(skip(connection, target))]
async fn handle_connection(connection: tokio::net::TcpStream, target: tokio::net::TcpStream) {
    tracing::info!("Handle Connection");

    let mut connection = Connection::new(
        networking::UnencryptedConnection::new(connection),
        bytes::BytesMut::with_capacity(4096),
    );
    let mut target_connection = Connection::new(
        networking::UnencryptedConnection::new(target),
        bytes::BytesMut::with_capacity(4096),
    );

    let packet = match connection
        .recv_legacy_packet(protocol::handshake::server::Handshaking::parse)
        .await
    {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("Receiving initial Packet: {:?}", e);
            return;
        }
    };
    tracing::info!("Received Handshaking Packet");

    let packet_data = match packet {
        protocol::packet::LegacyPacket::Ping => {
            tracing::error!("Legacy Ping");
            return;
        }
        protocol::packet::LegacyPacket::Actual(d) => d,
    };

    match &packet_data.next_state {
        protocol::handshake::server::NextState::Status => {
            tracing::info!("Status");

            target_connection
                .send_packet(&protocol::packet::Packet { inner: packet_data })
                .await
                .unwrap();

            status(connection, target_connection).await;
        }
        protocol::handshake::server::NextState::Login => {
            tracing::info!("Login");

            target_connection
                .send_packet(&protocol::packet::Packet { inner: packet_data })
                .await
                .unwrap();

            login(connection, target_connection).await;
        }
    };
}

#[tracing::instrument(skip(connection, target))]
async fn status<S, S2>(mut connection: Connection<S>, mut target: Connection<S2>)
where
    S: Transport,
    S2: Transport,
{
    loop {
        let packet = match connection
            .recv_packet(protocol::status::server::ServerBound::parse)
            .await
        {
            Ok(p) => p,
            Err(e) => {
                tracing::error!("Receiving Packet: {:?}", e);
                return;
            }
        };
        tracing::info!(?packet);

        match packet.inner {
            protocol::status::server::ServerBound::Status(s) => {
                tracing::info!(?s, "Status");

                target
                    .send_packet(&protocol::packet::Packet { inner: s })
                    .await
                    .unwrap();

                let response = target
                    .recv_packet(protocol::status::client::StatusResponse::parse)
                    .await
                    .unwrap();
                tracing::info!("Response-Packet: {:?}", response);

                connection.send_packet(&response).await.unwrap();

                tracing::info!("Send Response packet");
            }
            protocol::status::server::ServerBound::Ping(p) => {
                tracing::info!(?p, "Ping");

                target
                    .send_packet(&protocol::packet::Packet { inner: p })
                    .await
                    .unwrap();

                let response = target
                    .recv_packet(protocol::status::client::PingResponse::parse)
                    .await
                    .unwrap();
                tracing::info!("Response-Packet: {:?}", response);

                connection.send_packet(&response).await.unwrap();

                tracing::info!("Send Response Packet");
            }
        };
    }
}

#[tracing::instrument(skip(connection, target))]
async fn login<S, S2>(
    mut connection: Connection<networking::UnencryptedConnection<S>>,
    mut target: Connection<S2>,
) where
    S: core::marker::Unpin + tokio::io::AsyncRead + tokio::io::AsyncWrite,
    S2: Transport,
{
    let login_start_packet = connection
        .recv_packet(protocol::login::server::LoginStart::parse)
        .await
        .unwrap();
    tracing::info!(?login_start_packet);

    target.send_packet(&login_start_packet).await.unwrap();

    let response_packet = target
        .recv_packet(protocol::login::client::LoginSuccess::parse)
        .await
        .unwrap();
    tracing::info!("Response-Packet: {:?}", response_packet);

    connection.send_packet(&response_packet).await.unwrap();

    tracing::info!("Send Login Success");

    let packet = connection
        .recv_packet(protocol::login::server::LoginAck::parse)
        .await
        .unwrap();
    tracing::info!(?packet, "Login was Acknowledged");

    target.send_packet(&packet).await.unwrap();

    configuration(connection, target).await;
}

#[tracing::instrument(skip(connection, target))]
async fn configuration<S, S2>(mut connection: Connection<S>, mut target: Connection<S2>)
where
    S: Transport,
    S2: Transport,
{
    tracing::info!("Entering Configuration State of the connection");

    loop {
        tokio::select! {
            user_packet = connection.recv_rawpacket() => {
                let packet = match user_packet {
                    Ok(packet) => packet,
                    Err(e) => {
                        tracing::error!("Receiving from User: {:?}", e);
                        return;
                    }
                };

                match protocol::configuration::server::ConfigurationMessage::parse(packet.id, &packet.data) {
                    Ok(known) => {
                        tracing::info!("Client -> Server - {:#?}", known);
                    }
                    Err(_) => {
                        tracing::error!("Client -> Server - Unknown Packet: {:?}", packet.id);
                    }
                };

                target.send_rawpacket(&packet).await.unwrap();

                if packet.id.0 == <protocol::configuration::server::AckFinish as protocol::packet::PacketContent>::ID {
                    break;
                }
            }
            server_packet = target.recv_rawpacket() => {
                let packet = match server_packet {
                    Ok(packet) => packet,
                    Err(e) => {
                        tracing::error!("Receiving from Server: {:?}", e);
                        return;
                    }
                };

                match protocol::configuration::client::Configuration::parse(packet.id, &packet.data) {
                    Ok(known) => {
                        tracing::info!("Server -> Client - {:#?}", known);
                    }
                    Err(e) => {
                        tracing::error!("Server -> Client - Unknown Packet: {:?} - {:?}", packet.id, e);
                    }
                };

                connection.send_rawpacket(&packet).await.unwrap();
            }
        }
    }

    play(connection, target).await;
}

#[tracing::instrument(skip(connection, target))]
async fn play<S, S2>(mut connection: Connection<S>, mut target: Connection<S2>)
where
    S: Transport,
    S2: Transport,
{
    // TODO
    // How do we actually store/capture these packets for use afterwards

    loop {
        tokio::select! {
            user_packet = connection.recv_rawpacket() => {
                let packet = match user_packet {
                    Ok(packet) => packet,
                    Err(e) => {
                        tracing::error!("Receiving from User: {:?}", e);
                        return;
                    }
                };

                match protocol::play::server::Play::parse(packet.id, &packet.data) {
                    Ok((rem, packet)) if rem.is_empty() => {
                        // tracing::info!("[SERVER] {:#?}", packet);
                    }
                    Ok((rem, packet)) => {
                        tracing::error!("[CLIENT] {:?} Unparsed data: {:?}", packet, rem);
                    }
                    Err(e) => {
                        tracing::error!("[CLIENT] 0x{:02x} - Size: {:?} - Error: {:?}", packet.id.0, packet.data.len(), e);
                    }
                };

                if let Err(e) = target.send_rawpacket(&packet).await {
                    tracing::error!("Forwaring to server: {:?}", e);
                    return;
                }
            }
            server_packet = target.recv_rawpacket() => {
                let packet = match server_packet {
                    Ok(packet) => packet,
                    Err(e) => {
                        tracing::error!("Receiving from Server: {:?}", e);
                        return;
                    }
                };

                use protocol::packet::PacketContent;

                let id = packet.id;
                let data = &packet.data;
                match protocol::play::client::Play::parse(packet.id, &packet.data) {
                    Ok((rem, packet)) if rem.is_empty() => {
                        match packet {
                            protocol::play::client::Play::Login(p) => {
                                tracing::info!("Login Packet: {:?}", p);
                                let mut tmp = vec![0; 1024];
                                let serialized = p.serialize(&mut tmp).unwrap();

                                let (_, tmp) = protocol::play::client::Play::parse(id, &tmp).unwrap();
                                assert_eq!(protocol::play::client::Play::Login(p), tmp);
                                // assert_eq!(data, serialized);
                            }
                            other => {
                                // tracing::info!("[SERVER] {:#?}", packet);
                            }
                        };
                    }
                    Ok((rem, _packet)) => {
                        tracing::error!("[SERVER] 0x{:02x} - Unparsed Data: {:?}", packet.id.0, rem.len());
                    }
                    Err(e) => {
                        tracing::error!("[SERVER] 0x{:02x} - Size: {:?} - Error: {:?}", packet.id.0, packet.data.len(), e);
                    }
                };

                if let Err(e) = connection.send_rawpacket(&packet).await {
                    tracing::error!("Forwaring to client: {:?}", e);
                    return;
                }
            }
        }
    }
}
