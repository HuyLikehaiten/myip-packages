# myip-foo

Official Rust client for [myip.foo](https://myip.foo) - a free, privacy-focused IP lookup API.

[![Crates.io](https://img.shields.io/crates/v/myip-foo.svg)](https://crates.io/crates/myip-foo)
[![Documentation](https://docs.rs/myip-foo/badge.svg)](https://docs.rs/myip-foo)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- Async/await support with tokio
- Full type definitions with serde
- Dual-stack IPv4/IPv6 support
- Connection type detection
- No API key required

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
myip-foo = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust
use myip_foo::{get_ip, get_ip_data};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get plain IP
    let ip = get_ip().await?;
    println!("IP: {}", ip);

    // Get full data with geolocation
    let data = get_ip_data().await?;
    println!("IP: {}", data.ip);
    println!("City: {}", data.location.city);
    println!("Country: {}", data.location.country);
    println!("ISP: {}", data.network.isp);

    Ok(())
}
```

## API Reference

### Functions

#### `get_ip() -> Result<String>`
Returns your IP address as plain text.

#### `get_ip_data() -> Result<IpData>`
Returns full IP data including geolocation.

#### `get_dual_stack() -> Result<DualStackData>`
Returns both IPv4 and IPv6 addresses using dedicated endpoints.

#### `get_connection_type() -> Result<ConnectionTypeData>`
Detects if connection is residential, VPN, or datacenter.

#### `get_headers() -> Result<HashMap<String, String>>`
Returns all HTTP headers as seen by the server.

#### `get_user_agent() -> Result<String>`
Returns your user agent string.

### Types

```rust
pub struct IpData {
    pub ip: String,
    pub ip_type: String,           // "IPv4" or "IPv6"
    pub hostname: Option<String>,
    pub connection_type: Option<String>,  // "residential", "vpn", "datacenter", "tor"
    pub location: Location,
    pub network: Network,
    pub cloudflare: Cloudflare,
}

pub struct DualStackData {
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
}

pub struct ConnectionTypeData {
    pub ip: String,
    pub connection_type: String,  // "residential", "vpn", "datacenter", "unknown"
}
```

## Examples

### Get Dual-Stack IPs

```rust
use myip_foo::get_dual_stack;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dual = get_dual_stack().await?;

    if let Some(ipv4) = dual.ipv4 {
        println!("IPv4: {}", ipv4);
    }
    if let Some(ipv6) = dual.ipv6 {
        println!("IPv6: {}", ipv6);
    }

    Ok(())
}
```

### Check Connection Type

```rust
use myip_foo::get_connection_type;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection_type().await?;
    println!("Connection: {} ({})", conn.connection_type, conn.ip);

    Ok(())
}
```

## Dual-Stack Endpoints

The `get_dual_stack()` function uses dedicated subdomains:

- `ipv4.myip.foo/ip` - Returns IPv4 only (A record)
- `ipv6.myip.foo/ip` - Returns IPv6 only (AAAA record)

## Privacy

myip.foo does not log IP addresses or use cookies. See [Privacy Policy](https://myip.foo/privacy.html).

## Links

- **Website:** [myip.foo](https://myip.foo)
- **API Docs:** [myip.foo/api-docs](https://myip.foo/api-docs.html)
- **GitHub:** [myip-packages](https://github.com/virtualox/myip-packages)

## License

MIT License - see [LICENSE](../../LICENSE) for details.
