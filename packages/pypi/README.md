# myip-foo

Official Python client for [myip.foo](https://myip.foo) - a free, privacy-focused IP lookup API.

[![PyPI version](https://img.shields.io/pypi/v/myip-foo.svg)](https://pypi.org/project/myip-foo/)
[![Python versions](https://img.shields.io/pypi/pyversions/myip-foo.svg)](https://pypi.org/project/myip-foo/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- Full type hints (PEP 561)
- Zero dependencies (async requires aiohttp)
- Dual-stack IPv4/IPv6 support
- Connection type detection
- No API key required

## Installation

```bash
pip install myip-foo

# With async support
pip install myip-foo[async]
```

## Quick Start

```python
from myip_foo import get_ip, get_ip_data, get_dual_stack

# Get plain IP
ip = get_ip()
print(ip)  # "203.0.113.42"

# Get full data with geolocation
data = get_ip_data()
print(data['ip'])                    # "203.0.113.42"
print(data['location']['city'])      # "Amsterdam"
print(data['location']['country'])   # "NL"
print(data['network']['isp'])        # "KPN B.V."
print(data['connectionType'])        # "residential"

# Get both IPv4 and IPv6
dual = get_dual_stack()
print(dual['ipv4'])  # "203.0.113.42"
print(dual['ipv6'])  # "2001:db8::1" or None
```

## API Reference

### Functions

#### `get_ip() -> str`
Returns your IP address as plain text.

#### `get_ip_data() -> IPData`
Returns full IP data including geolocation.

#### `get_dual_stack(timeout: int = 5) -> DualStackData`
Returns both IPv4 and IPv6 addresses.

#### `get_connection_type() -> ConnectionTypeData`
Detects if connection is residential, VPN, or datacenter.

#### `get_headers() -> dict[str, str]`
Returns all HTTP headers as seen by the server.

#### `get_user_agent() -> str`
Returns your user agent string.

### Async Functions

Install with `pip install myip-foo[async]` to use async versions:

```python
import asyncio
from myip_foo import get_ip_async, get_ip_data_async, get_dual_stack_async

async def main():
    ip = await get_ip_async()
    data = await get_ip_data_async()
    dual = await get_dual_stack_async()
    print(f"IP: {ip}, City: {data['location']['city']}")

asyncio.run(main())
```

### Types

```python
class IPData(TypedDict):
    ip: str
    type: str  # "IPv4" or "IPv6"
    hostname: Optional[str]
    connectionType: Optional[str]  # "residential", "vpn", "datacenter", "tor"
    location: Location
    network: Network
    cloudflare: Cloudflare

class DualStackData(TypedDict):
    ipv4: Optional[str]
    ipv6: Optional[str]

class ConnectionTypeData(TypedDict):
    ip: str
    type: str  # "residential", "vpn", "datacenter", "unknown"
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
