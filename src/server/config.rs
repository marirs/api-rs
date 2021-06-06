/// Parse Settings from a configuration yaml file
use serde::{Deserialize, Deserializer};

use std::{net::IpAddr, path::Path};

type Error = String;

const SRV_ADDR: &str = "127.0.0.1";
const SRV_PORT: usize = 8080;
const SRV_KEEP_ALIVE: usize = 60;
const SRV_FORMS_LIMIT: usize = 1024 * 256;
const SRV_JSON_LIMIT: usize = 1024 * 256;
const SRV_SECRET_KEY: &str = "t/xZkYvxfC8CSfTSH9ANiIR9t1SvLHqOYZ7vH4fp11s=";

const SSL_ENABLED: bool = false;
const SSL_GENERATE_SELF_SIGNED: bool = true;
const SSL_KEY_FILE: &str = "./private/key";
const SSL_CERT_FILE: &str = "./private/cert";

/// Rocket API Server parameters
#[derive(Deserialize, Clone, Debug, Default)]
pub struct Settings {
    /// Server config related parameters
    #[serde(default)]
    pub server: ServerConfig,

    /// Server config related parameters
    #[serde(default, deserialize_with = "configure_ssl")]
    pub ssl: Option<SslConfig>,

    // Any more sections from the config.yml goes in here

}

impl Settings {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        //! Read configuration settings from yaml file
        //!
        //! ## Example usage
        //! ```ignore
        //! Settings::from_file("config.yml");
        //! ```
        //!
        let mut cfg = config::Config::default();
        match cfg.merge(config::File::with_name(path.as_ref().to_str().unwrap())) {
            Ok(m) => m
                .merge(config::File::with_name(path.as_ref().to_str().unwrap()))
                .unwrap(),
            Err(e) => return Err(e.to_string().replace("\"", "")),
        };
        Ok(cfg.try_into().unwrap())
    }
}

/// Rocket Server params
#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
    /// Server Ip Address to start Rocket API Server
    #[serde(default = "default_server_host")]
    pub host: IpAddr,
    /// Server port to listen Rocket API Server
    #[serde(default = "default_server_port")]
    pub port: usize,
    /// Server Keep Alive
    #[serde(default = "default_server_keep_alive")]
    pub keep_alive: usize,
    /// Forms limitation
    #[serde(default = "default_server_forms_limit")]
    pub forms_limit: usize,
    /// JSON transfer limitation
    #[serde(default = "default_server_json_limit")]
    pub json_limit: usize,
    /// Api Server Secret key
    #[serde(default = "default_server_secret_key")]
    pub secret_key: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: SRV_ADDR.parse().unwrap(),
            port: SRV_PORT,
            keep_alive: SRV_KEEP_ALIVE,
            forms_limit: SRV_FORMS_LIMIT,
            json_limit: SRV_JSON_LIMIT,
            secret_key: SRV_SECRET_KEY.into(),
        }
    }
}

/// Server SSL params
#[derive(Deserialize, Clone, Debug)]
pub struct SslConfig {
    /// Enabled: yes/no
    #[serde(default = "default_ssl_enabled")]
    pub enabled: bool,
    /// Let the server generate a self-signed pair for you: yes/no
    #[serde(default = "default_ssl_self_signed")]
    generate_self_signed: bool,
    /// key file (if generate_self_signed is `NO`)
    #[serde(default = "default_ssl_key_file")]
    key_file: String,
    /// certificate pem file (if generate_self_signed is `NO`)
    #[serde(default = "default_ssl_cert_file")]
    cert_file: String,

    // Not to be included in config file
    // hidden and for use with rocket app
    pub pem_certificate: Option<String>,
    pub pem_private_key: Option<String>,
}

impl Default for SslConfig {
    fn default() -> Self {
        Self {
            enabled: SSL_ENABLED,
            generate_self_signed: SSL_GENERATE_SELF_SIGNED,
            key_file: SSL_KEY_FILE.into(),
            cert_file: SSL_CERT_FILE.into(),
            pem_certificate: None,
            pem_private_key: None
        }
    }
}

// All Server defaults
fn default_server_host() -> IpAddr {
    SRV_ADDR.parse().unwrap()
}

fn default_server_port() -> usize {
    SRV_PORT
}

fn default_server_keep_alive() -> usize {
    SRV_KEEP_ALIVE
}

fn default_server_forms_limit() -> usize {
    SRV_FORMS_LIMIT
}

fn default_server_json_limit() -> usize {
    SRV_JSON_LIMIT
}

fn default_server_secret_key() -> String {
    SRV_SECRET_KEY.into()
}

// All SSL config defaults
fn default_ssl_enabled() -> bool {
    SSL_ENABLED
}

fn default_ssl_self_signed() -> bool {
    SSL_GENERATE_SELF_SIGNED
}

fn default_ssl_key_file() -> String {
    SSL_KEY_FILE.into()
}

fn default_ssl_cert_file() -> String {
    SSL_CERT_FILE.into()
}

fn configure_ssl<'de, D>(deserializer: D) -> Result<Option<SslConfig>, D::Error>
    where
        D: Deserializer<'de>,
{

}