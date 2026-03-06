//! Rudimentary Quinn QUIC echo server.
//! Run on your home PC. Listens on 0.0.0.0:4433 so it's reachable from the network (with port forwarding).

use std::net::SocketAddr;

use quinn::{Endpoint, Incoming, ServerConfig};
use rcgen::{generate_simple_self_signed, CertifiedKey};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("failed to install rustls crypto provider");

    let listen_addr: SocketAddr = "0.0.0.0:4433".parse()?;
    run(listen_addr)
}

#[tokio::main]
async fn run(listen_addr: SocketAddr) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let server_config = configure_server()?;
    let endpoint = Endpoint::server(server_config, listen_addr)?;

    println!("Ember QUIC server listening on {}", endpoint.local_addr()?);
    println!("Connect from client with: cargo run -p ember-client -- <host>:4433");

    while let Some(incoming) = endpoint.accept().await {
        tokio::spawn(async move {
            if let Err(e) = handle_connection(incoming).await {
                eprintln!("connection error: {e}");
            }
        });
    }

    Ok(())
}

fn configure_server() -> Result<ServerConfig, Box<dyn std::error::Error + Send + Sync>> {
    // Generate self-signed cert. For remote access, add your hostname/IP to the list.
    let CertifiedKey { cert, key_pair } =
        generate_simple_self_signed(vec!["localhost".to_string(), "127.0.0.1".to_string()])?;
    let cert_der = CertificateDer::from(cert);
    let key = PrivatePkcs8KeyDer::from(key_pair.serialize_der());
    let key_der = PrivateKeyDer::try_from(key)?;

    let server_config = ServerConfig::with_single_cert(vec![cert_der], key_der)?;
    Ok(server_config)
}

async fn handle_connection(incoming: Incoming) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let connection = incoming.await?;
    let remote = connection.remote_address();
    println!("Connection from {}", remote);

    loop {
        let (mut send, mut recv) = match connection.accept_bi().await {
            Ok(stream) => stream,
            Err(quinn::ConnectionError::ApplicationClosed(_)) => break,
            Err(e) => return Err(e.into()),
        };

        tokio::spawn(async move {
            if let Err(e) = echo_stream(&mut send, &mut recv).await {
                eprintln!("stream error: {e}");
            }
        });
    }

    Ok(())
}

async fn echo_stream(
    send: &mut quinn::SendStream,
    recv: &mut quinn::RecvStream,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let data = recv.read_to_end(64 * 1024).await?;
    println!("  received {} bytes (echoing back)", data.len());
    send.write_all(&data).await?;
    send.finish()?;
    Ok(())
}
