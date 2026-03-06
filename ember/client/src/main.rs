//! Desktop binary for the ember QUIC client.

use std::net::SocketAddr;

fn main() -> Result<(), String> {
    let server_addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:4433".to_string());
    let server_addr: SocketAddr = server_addr.parse().map_err(|e: std::net::AddrParseError| e.to_string())?;

    println!("Connecting to {}...", server_addr);
    let response = ember_native::connect_echo(server_addr)?;
    println!("Echo response: {}", response);
    Ok(())
}
