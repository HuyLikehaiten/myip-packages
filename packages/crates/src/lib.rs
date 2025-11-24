//! # myip-foo
//!
//! Official Rust client for [myip.foo](https://myip.foo) - a free, privacy-focused IP lookup API.
//!
//! ## Features
//!
//! - Async/await support with tokio
//! - Full type definitions with serde
//! - Dual-stack IPv4/IPv6 support
//! - Connection type detection
//! - No API key required
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use myip_foo::{get_ip, get_ip_data};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Get plain IP
//!     let ip = get_ip().await?;
//!     println!("IP: {}", ip);
//!
//!     // Get full data
//!     let data = get_ip_data().await?;
//!     println!("City: {}", data.location.city);
//!
//!     Ok(())
//! }
//! ```

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const BASE_URL: &str = "https://myip.foo";
const IPV4_URL: &str = "https://ipv4.myip.foo";
const IPV6_URL: &str = "https://ipv6.myip.foo";

/// Location information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub country: String,
    pub city: String,
    pub region: String,
    pub postal_code: String,
    pub timezone: String,
    pub latitude: String,
    pub longitude: String,
}

/// Network information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub asn: i64,
    pub isp: String,
}

/// Cloudflare information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cloudflare {
    pub colo: String,
    pub ray: String,
}

/// Full IP data including geolocation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpData {
    pub ip: String,
    #[serde(rename = "type")]
    pub ip_type: String,
    pub hostname: Option<String>,
    pub connection_type: Option<String>,
    pub location: Location,
    pub network: Network,
    pub cloudflare: Cloudflare,
}

/// Dual-stack IPv4/IPv6 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualStackData {
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
}

/// Connection type data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionTypeData {
    pub ip: String,
    #[serde(rename = "type")]
    pub connection_type: String,
}

/// IP response from dual-stack endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
struct IpResponse {
    ip: String,
}

/// Error type for myip-foo operations
#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    Parse(serde_json::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Request(e) => write!(f, "Request error: {}", e),
            Error::Parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Request(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Parse(err)
    }
}

/// Result type for myip-foo operations
pub type Result<T> = std::result::Result<T, Error>;

fn create_client() -> reqwest::Result<Client> {
    Client::builder()
        .user_agent("myip-foo/1.0.0")
        .timeout(Duration::from_secs(10))
        .build()
}

/// Get your IP address as plain text.
///
/// # Example
///
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let ip = myip_foo::get_ip().await?;
/// println!("My IP: {}", ip);
/// # Ok(())
/// # }
/// ```
pub async fn get_ip() -> Result<String> {
    let client = create_client()?;
    let response = client
        .get(format!("{}/plain", BASE_URL))
        .send()
        .await?
        .text()
        .await?;
    Ok(response.trim().to_string())
}

/// Get full IP data including geolocation.
///
/// # Example
///
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let data = myip_foo::get_ip_data().await?;
/// println!("IP: {}", data.ip);
/// println!("City: {}", data.location.city);
/// println!("ISP: {}", data.network.isp);
/// # Ok(())
/// # }
/// ```
pub async fn get_ip_data() -> Result<IpData> {
    let client = create_client()?;
    let response = client
        .get(format!("{}/api", BASE_URL))
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}

/// Get both IPv4 and IPv6 addresses.
///
/// Uses dedicated endpoints that bypass Cloudflare's dual-stack routing.
///
/// # Example
///
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let dual = myip_foo::get_dual_stack().await?;
/// if let Some(ipv4) = dual.ipv4 {
///     println!("IPv4: {}", ipv4);
/// }
/// if let Some(ipv6) = dual.ipv6 {
///     println!("IPv6: {}", ipv6);
/// }
/// # Ok(())
/// # }
/// ```
pub async fn get_dual_stack() -> Result<DualStackData> {
    let client = Client::builder()
        .user_agent("myip-foo/1.0.0")
        .timeout(Duration::from_secs(5))
        .build()?;

    let ipv4 = match client.get(format!("{}/ip", IPV4_URL)).send().await {
        Ok(resp) => match resp.json::<IpResponse>().await {
            Ok(data) => Some(data.ip),
            Err(_) => None,
        },
        Err(_) => None,
    };

    let ipv6 = match client.get(format!("{}/ip", IPV6_URL)).send().await {
        Ok(resp) => match resp.json::<IpResponse>().await {
            Ok(data) => Some(data.ip),
            Err(_) => None,
        },
        Err(_) => None,
    };

    Ok(DualStackData { ipv4, ipv6 })
}

/// Get connection type (residential, vpn, datacenter).
///
/// # Example
///
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let conn = myip_foo::get_connection_type().await?;
/// println!("Type: {}", conn.connection_type);
/// # Ok(())
/// # }
/// ```
pub async fn get_connection_type() -> Result<ConnectionTypeData> {
    let client = create_client()?;
    let response = client
        .get(format!("{}/api/connection-type", BASE_URL))
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}

/// Get all HTTP headers as seen by the server.
///
/// # Example
///
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let headers = myip_foo::get_headers().await?;
/// for (key, value) in headers {
///     println!("{}: {}", key, value);
/// }
/// # Ok(())
/// # }
/// ```
pub async fn get_headers() -> Result<std::collections::HashMap<String, String>> {
    let client = create_client()?;
    let response = client
        .get(format!("{}/headers", BASE_URL))
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}

/// Get your user agent string.
///
/// # Example
///
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let ua = myip_foo::get_user_agent().await?;
/// println!("User-Agent: {}", ua);
/// # Ok(())
/// # }
/// ```
pub async fn get_user_agent() -> Result<String> {
    let client = create_client()?;
    let response: serde_json::Value = client
        .get(format!("{}/user-agent", BASE_URL))
        .send()
        .await?
        .json()
        .await?;

    Ok(response["userAgent"]
        .as_str()
        .unwrap_or("")
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_ip() {
        let result = get_ip().await;
        assert!(result.is_ok());
        let ip = result.unwrap();
        assert!(!ip.is_empty());
    }
}
