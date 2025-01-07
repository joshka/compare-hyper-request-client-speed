use std::{sync::Arc, time::Instant};

use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper_util::{
    client::legacy::{connect::Connect, Client},
    rt::TokioExecutor,
};
use rustls::{
    client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
    SignatureScheme,
};
use tracing::{info, instrument};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    tracing_subscriber::fmt().init();

    let reqwest_client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    let config = rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(NoVerifier))
        .with_no_client_auth();
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_tls_config(config)
        .https_only()
        .enable_http1()
        .build();
    let hyper_client = Client::builder(TokioExecutor::new()).build(connector);

    use_hyper(&hyper_client).await?;
    use_hyper(&hyper_client).await?;
    use_hyper(&hyper_client).await?;
    use_reqwest(&reqwest_client).await?;
    use_reqwest(&reqwest_client).await?;
    use_reqwest(&reqwest_client).await?;
    use_hyper(&hyper_client).await?;
    use_hyper(&hyper_client).await?;
    use_hyper(&hyper_client).await?;
    use_reqwest(&reqwest_client).await?;
    use_reqwest(&reqwest_client).await?;
    use_reqwest(&reqwest_client).await?;
    use_hyper(&hyper_client).await?;
    use_hyper(&hyper_client).await?;
    use_hyper(&hyper_client).await?;

    Ok(())
}

#[instrument(name = "reqwest", skip_all)]
async fn use_reqwest(client: &reqwest::Client) -> color_eyre::Result<()> {
    let start = Instant::now();
    let response = client.get("https://127.0.0.1:3000").send().await?;
    let head_time = start.elapsed();
    let _body = response.text().await?;
    let body_time = start.elapsed() - head_time;
    info!("Time. Head: {:>7.1?}, Body: {:>7.1?}", head_time, body_time);
    Ok(())
}

#[instrument(name = "__hyper", skip_all)]
async fn use_hyper<C>(client: &Client<C, Full<Bytes>>) -> color_eyre::Result<()>
where
    C: Connect + Clone + Send + Sync + 'static,
{
    let url = "https://127.0.0.1:3000".parse()?;
    let start = Instant::now();
    let response = client.get(url).await?;
    let head_time = start.elapsed();
    let _body = response.into_body().collect().await?.to_bytes();
    let body_time = start.elapsed() - head_time;
    info!("Time. Head: {:>7.1?}, Body: {:>7.1?}", head_time, body_time);
    Ok(())
}

#[derive(Debug)]
struct NoVerifier;

impl ServerCertVerifier for NoVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::pki_types::CertificateDer<'_>,
        _intermediates: &[rustls::pki_types::CertificateDer<'_>],
        _server_name: &rustls::pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        todo!()
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            SignatureScheme::RSA_PKCS1_SHA1,
            SignatureScheme::ECDSA_SHA1_Legacy,
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::RSA_PKCS1_SHA384,
            SignatureScheme::ECDSA_NISTP384_SHA384,
            SignatureScheme::RSA_PKCS1_SHA512,
            SignatureScheme::ECDSA_NISTP521_SHA512,
            SignatureScheme::RSA_PSS_SHA256,
            SignatureScheme::RSA_PSS_SHA384,
            SignatureScheme::RSA_PSS_SHA512,
            SignatureScheme::ED25519,
            SignatureScheme::ED448,
        ]
    }
}
