use tracing_subscriber::layer::SubscriberExt;

use server::Connection;

fn main() {
    println!("Hello, world!");

    let fmt_layer = tracing_subscriber::fmt::layer().with_ansi(false);
    tracing::subscriber::set_global_default(
        tracing_subscriber::Registry::default().with(fmt_layer),
    )
    .unwrap();

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

    while let Ok((connection, addr)) = listener.accept().await {
        tracing::info!(?addr);

        tokio::spawn(handle_connection(connection));
    }

    tracing::error!("Stopped");
}

#[tracing::instrument(skip(connection))]
async fn handle_connection(connection: tokio::net::TcpStream) {
    use server::{recv_packet, recv_legacy_packet};

    tracing::info!("Handle Connection");

    let mut buffer = bytes::BytesMut::with_capacity(4096);

    let mut connection = server::UnencryptedConnection::new(connection);

    let packet = match recv_legacy_packet(
        &mut connection,
        &mut buffer,
        protocol::handshake::server::Handshaking::parse,
    )
    .await {
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

    match packet_data.next_state {
        protocol::handshake::server::NextState::Status => {
            tracing::info!("Status");

            status(connection, buffer).await;
        }
        protocol::handshake::server::NextState::Login => {
            tracing::info!("Login");

            let login_start_packet = recv_packet(
                &mut connection,
                &mut buffer,
                protocol::login::server::LoginStart::parse,
            )
            .await
            .unwrap();
            tracing::info!(?login_start_packet);

            let private_key = openssl::rsa::Rsa::generate(1024).unwrap();
            let pub_key = openssl::rsa::Rsa::from_public_components(
                (private_key.n()).to_owned().unwrap(),
                (private_key.e()).to_owned().unwrap(),
            )
            .unwrap();
            let pub_key_encoded = protocol::login::asn1_encode_key(pub_key.n(), pub_key.e());

            let server_verify_token = [123, 71, 83, 90];

            let encryption_packet = protocol::packet::Packet {
                inner: protocol::login::client::EncryptionRequest {
                    server_id: protocol::general::PString("".into()),
                    pubkey: pub_key_encoded.clone(),
                    verifytoken: server_verify_token.to_vec(),
                },
            };
            connection.send_packet(&encryption_packet).await.unwrap();

            tracing::info!("Send EncryptionRequest");

            let encryption_response = match recv_packet(
                &mut connection,
                &mut buffer,
                protocol::login::server::EncryptionResponse::parse,
            )
            .await
            {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!("Receiving EncryptionResponse: {:?}", e);
                    loop {
                        tokio::task::yield_now().await;
                    }
                }
            };

            let mut shared_secret = [0; 128];
            let written = private_key
                .private_decrypt(
                    &encryption_response.inner.shared_secret,
                    &mut shared_secret,
                    openssl::rsa::Padding::PKCS1,
                )
                .unwrap();
            let shared_secret = &shared_secret[..written];

            let mut client_verify_token = [0; 128];
            let written = private_key
                .private_decrypt(
                    &encryption_response.inner.verify_token,
                    &mut client_verify_token,
                    openssl::rsa::Padding::PKCS1,
                )
                .unwrap();
            let client_verify_token = &client_verify_token[..written];

            if client_verify_token != &server_verify_token {
                tracing::error!("Verify Tokens dont match");
            }

            let http_client = reqwest::Client::new();

            let url = format!(
                "https://sessionserver.mojang.com/session/minecraft/profile/{:x}?unsigned=false",
                login_start_packet.inner.uuid
            );
            let req = http_client.request(reqwest::Method::GET, url);
            let session_resp = req.send().await.unwrap();
            let raw_response = session_resp.text().await.unwrap();
            let session_json: server::ProfileResponse =
                serde_json::from_str(&raw_response).unwrap();

            let session_uuid = uuid::Uuid::parse_str(&session_json.id).unwrap();

            let response_packet = protocol::packet::Packet {
                inner: protocol::login::client::LoginSuccess {
                    uuid: session_uuid.as_u128(),
                    name: protocol::general::PString(std::borrow::Cow::Owned(
                        session_json.name.clone(),
                    )),
                    properites: Vec::new(),
                },
            };

            let mut connection = connection.encrypt(
                openssl::symm::Crypter::new(
                    openssl::symm::Cipher::aes_128_cfb8(),
                    openssl::symm::Mode::Decrypt,
                    shared_secret,
                    Some(shared_secret),
                )
                .unwrap(),
                openssl::symm::Crypter::new(
                    openssl::symm::Cipher::aes_128_cfb8(),
                    openssl::symm::Mode::Encrypt,
                    shared_secret,
                    Some(shared_secret),
                )
                .unwrap(),
            );

            connection.send_packet(&response_packet).await.unwrap();

            tracing::info!("Send Login Success");

            let packet = recv_packet(
                &mut connection,
                &mut buffer,
                protocol::login::server::LoginAck::parse,
            )
            .await
            .unwrap();
            tracing::info!(?packet);
        }
    };
}

async fn status<S>(mut connection: S, mut buffer: bytes::BytesMut) where S: server::Connection {
    use server::recv_packet;

    loop {
        let packet = match recv_packet(
                &mut connection,
                &mut buffer,
                protocol::status::server::ServerBound::parse,
            )
            .await {
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

                    let response_content = protocol::status::client::StatusResponseContent {
                        version: protocol::status::client::StatusVersion {
                            name: "1.20.6".into(),
                            protocol: 766,
                        },
                        players: protocol::status::client::StatusPlayers {
                            max: 69,
                            online: 0,
                            sample: Vec::new(),
                        },
                        description: protocol::status::client::StatusDescription {
                            text: "custom server implemenation".into(),
                        },
                        previews_chat: false,
                        enforces_secure_chat: false,
                    };

                    let response_packet = protocol::packet::Packet {
                        inner: protocol::status::client::StatusResponse::new(&response_content),
                    };
                    connection.send_packet(&response_packet).await.unwrap();

                    tracing::info!("Send Response packet");
                }
                protocol::status::server::ServerBound::Ping(p) => {
                    tracing::info!(?p, "Ping");

                    let response_packet = protocol::packet::Packet {
                    inner: protocol::status::client::PingResponse {
                        payload: p.payload,
                    },
                };
                connection.send_packet(&response_packet).await.unwrap();

                    tracing::info!("Send Response Packet");
                }
            };
    }
}
