"""
myip-foo - Official Python client for myip.foo API

A free, privacy-focused IP lookup API with geolocation.
"""

import asyncio
from typing import Optional, TypedDict
from urllib.request import urlopen, Request
from urllib.error import URLError
import json

__version__ = "1.0.0"

BASE_URL = "https://myip.foo"
IPV4_URL = "https://ipv4.myip.foo"
IPV6_URL = "https://ipv6.myip.foo"


class Location(TypedDict):
    country: str
    city: str
    region: str
    postalCode: str
    timezone: str
    latitude: str
    longitude: str


class Network(TypedDict):
    asn: int
    isp: str


class Cloudflare(TypedDict):
    colo: str
    ray: str


class IPData(TypedDict):
    ip: str
    type: str
    hostname: Optional[str]
    connectionType: Optional[str]
    location: Location
    network: Network
    cloudflare: Cloudflare


class DualStackData(TypedDict):
    ipv4: Optional[str]
    ipv6: Optional[str]


class ConnectionTypeData(TypedDict):
    ip: str
    type: str


def _fetch(url: str, timeout: int = 10) -> str:
    """Internal fetch helper."""
    req = Request(url, headers={"User-Agent": f"myip-foo/{__version__}"})
    with urlopen(req, timeout=timeout) as response:
        return response.read().decode("utf-8")


def get_ip() -> str:
    """
    Get your IP address as plain text.

    Returns:
        str: Your IP address

    Example:
        >>> from myip_foo import get_ip
        >>> ip = get_ip()
        >>> print(ip)
        '203.0.113.42'
    """
    return _fetch(f"{BASE_URL}/plain").strip()


def get_ip_data() -> IPData:
    """
    Get full IP data including geolocation.

    Returns:
        IPData: Dictionary with ip, location, network, etc.

    Example:
        >>> from myip_foo import get_ip_data
        >>> data = get_ip_data()
        >>> print(data['ip'])
        '203.0.113.42'
        >>> print(data['location']['city'])
        'Amsterdam'
    """
    response = _fetch(f"{BASE_URL}/api")
    return json.loads(response)


def get_dual_stack(timeout: int = 5) -> DualStackData:
    """
    Get both IPv4 and IPv6 addresses.

    Uses dedicated endpoints that bypass Cloudflare's dual-stack routing.

    Args:
        timeout: Timeout in seconds for each request (default: 5)

    Returns:
        DualStackData: Dictionary with ipv4 and ipv6 (None if not available)

    Example:
        >>> from myip_foo import get_dual_stack
        >>> data = get_dual_stack()
        >>> print(data['ipv4'])
        '203.0.113.42'
        >>> print(data['ipv6'])
        '2001:db8::1'
    """
    ipv4 = None
    ipv6 = None

    try:
        response = _fetch(f"{IPV4_URL}/ip", timeout)
        data = json.loads(response)
        ipv4 = data.get("ip")
    except (URLError, TimeoutError, json.JSONDecodeError):
        pass

    try:
        response = _fetch(f"{IPV6_URL}/ip", timeout)
        data = json.loads(response)
        ipv6 = data.get("ip")
    except (URLError, TimeoutError, json.JSONDecodeError):
        pass

    return {"ipv4": ipv4, "ipv6": ipv6}


def get_connection_type() -> ConnectionTypeData:
    """
    Get connection type (residential, vpn, datacenter).

    Returns:
        ConnectionTypeData: Dictionary with connectionType, provider, asn

    Example:
        >>> from myip_foo import get_connection_type
        >>> data = get_connection_type()
        >>> print(data['connectionType'])
        'residential'
    """
    response = _fetch(f"{BASE_URL}/api/connection-type")
    return json.loads(response)


def get_headers() -> dict[str, str]:
    """
    Get all HTTP headers as seen by the server.

    Returns:
        dict: Dictionary of header names and values
    """
    response = _fetch(f"{BASE_URL}/headers")
    return json.loads(response)


def get_user_agent() -> str:
    """
    Get your user agent string.

    Returns:
        str: Your user agent string
    """
    response = _fetch(f"{BASE_URL}/user-agent")
    data = json.loads(response)
    return data.get("userAgent", "")


# Async versions for use with asyncio
try:
    import aiohttp

    async def get_ip_async() -> str:
        """Async version of get_ip()."""
        async with aiohttp.ClientSession() as session:
            async with session.get(f"{BASE_URL}/plain") as response:
                text = await response.text()
                return text.strip()

    async def get_ip_data_async() -> IPData:
        """Async version of get_ip_data()."""
        async with aiohttp.ClientSession() as session:
            async with session.get(f"{BASE_URL}/api") as response:
                return await response.json()

    async def get_dual_stack_async(timeout: int = 5) -> DualStackData:
        """Async version of get_dual_stack()."""
        async with aiohttp.ClientSession(timeout=aiohttp.ClientTimeout(total=timeout)) as session:
            ipv4 = None
            ipv6 = None

            try:
                async with session.get(f"{IPV4_URL}/ip") as response:
                    ipv4 = (await response.text()).strip()
            except:
                pass

            try:
                async with session.get(f"{IPV6_URL}/ip") as response:
                    ipv6 = (await response.text()).strip()
            except:
                pass

            return {"ipv4": ipv4, "ipv6": ipv6}

except ImportError:
    # aiohttp not installed, async functions not available
    pass
