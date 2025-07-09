use std::net::SocketAddr;

#[derive(Clone, Debug, Default, serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_private_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_private_key: Option<String>,

    /// Configure the address to listen on for https
    ///
    /// Defaults to `0.0.0.0:443` if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_address: Option<SocketAddr>,

    /// Configure the address to listen on for http
    ///
    /// Defaults to `0.0.0.0:80` if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_address: Option<SocketAddr>,

    /// Configure the address to listen on for http metrics
    ///
    /// Defaults to `127.0.0.1:9180` if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_http_address: Option<SocketAddr>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sui_chain_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitcoin_chain_id: Option<String>,
}

impl Config {
    pub fn load(path: &std::path::Path) -> Result<Self, anyhow::Error> {
        let file = std::fs::read(path)?;
        toml::from_slice(&file).map_err(Into::into)
    }

    pub fn save(&self, path: &std::path::Path) -> Result<(), anyhow::Error> {
        let toml = toml::to_string(self)?;
        std::fs::write(path, toml).map_err(Into::into)
    }

    pub fn protocol_private_key(&self) -> Result<ed25519_dalek::SigningKey, String> {
        todo!()
    }

    pub fn tls_private_key(&self) -> Result<ed25519_dalek::SigningKey, anyhow::Error> {
        use ed25519_dalek::pkcs8::DecodePrivateKey;

        let raw = self
            .tls_private_key
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("no tls_private_key configured"))?;

        if let Ok(private_key) = ed25519_dalek::SigningKey::read_pkcs8_pem_file(raw) {
            Ok(private_key)
        } else if let Ok(private_key) = ed25519_dalek::SigningKey::read_pkcs8_der_file(raw) {
            Ok(private_key)
        } else if let Ok(private_key) = ed25519_dalek::SigningKey::from_pkcs8_pem(raw) {
            Ok(private_key)
        } else {
            // maybe some other format?
            Err(anyhow::anyhow!("unable to load tls_private_key"))
        }
    }

    pub fn https_address(&self) -> SocketAddr {
        self.https_address
            .unwrap_or_else(|| SocketAddr::from(([0, 0, 0, 0], 443)))
    }

    pub fn http_address(&self) -> SocketAddr {
        self.http_address
            .unwrap_or_else(|| SocketAddr::from(([0, 0, 0, 0], 80)))
    }

    pub fn metrics_http_address(&self) -> SocketAddr {
        self.metrics_http_address
            .unwrap_or_else(|| SocketAddr::from(([127, 0, 0, 1], 9180)))
    }

    pub fn sui_chain_id(&self) -> &str {
        self.sui_chain_id
            .as_deref()
            .unwrap_or("4btiuiMPvEENsttpZC7CZ53DruC3MAgfznDbASZ7DR6S")
    }

    pub fn bitcoin_chain_id(&self) -> &str {
        self.bitcoin_chain_id
            .as_deref()
            .unwrap_or("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f")
    }
}
