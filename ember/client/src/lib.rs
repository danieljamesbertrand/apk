//! Ember QUIC client library. Used by the desktop binary and Android JNI.

#[cfg(feature = "android")]
mod jni;

use std::net::SocketAddr;
use std::sync::Arc;

use quinn::{ClientConfig, Endpoint};
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::crypto::CryptoProvider;
use rustls::{ClientConfig as RustlsClientConfig, DigitallySignedStruct, SignatureScheme};

/// Skip certificate verification (development only - do not use in production!)
#[derive(Debug)]
struct SkipServerVerification(Arc<CryptoProvider>);

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self(Arc::new(
            rustls::crypto::ring::default_provider(),
        )))
    }
}

impl ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls12_signature(
            message,
            cert,
            dss,
            &self.0.signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(
            message,
            cert,
            dss,
            &self.0.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.0
            .signature_verification_algorithms
            .supported_schemes()
    }
}

fn configure_client(provider: Arc<CryptoProvider>) -> Result<ClientConfig, Box<dyn std::error::Error + Send + Sync>> {
    let crypto = RustlsClientConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()
        .map_err(|e| format!("{:?}", e))?
        .dangerous()
        .with_custom_certificate_verifier(SkipServerVerification::new())
        .with_no_client_auth();

    let client_config = ClientConfig::new(Arc::new(
        quinn::crypto::rustls::QuicClientConfig::try_from(crypto)?,
    ));
    Ok(client_config)
}

/// Connect to the ember QUIC server and return the echo response.
/// Blocking wrapper around the async logic.
pub fn connect_echo(server_addr: SocketAddr) -> Result<String, String> {
    let provider = Arc::new(rustls::crypto::ring::default_provider());

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| e.to_string())?;

    rt.block_on(async_run(server_addr, provider)).map_err(|e| e.to_string())
}

async fn async_run(server_addr: SocketAddr, provider: Arc<CryptoProvider>) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client_config = configure_client(provider)?;
    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
    endpoint.set_default_client_config(client_config);

    let connection = endpoint
        .connect(server_addr, "localhost")?
        .await
        .map_err(|e| format!("connection failed: {e}"))?;

    let (mut send, mut recv) = connection.open_bi().await?;
    let message = b"Hello from ember Android!";
    send.write_all(message).await?;
    send.finish()?;

    let response = recv.read_to_end(64 * 1024).await?;
    let result = String::from_utf8_lossy(&response).to_string();

    connection.close(0u32.into(), b"done");
    endpoint.wait_idle().await;

    Ok(result)
}
