# myip-foo

Official JavaScript/TypeScript client for [myip.foo](https://myip.foo) - a free, privacy-focused IP lookup API.

[![npm version](https://img.shields.io/npm/v/myip-foo.svg)](https://www.npmjs.com/package/myip-foo)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- Full TypeScript support
- Works in Node.js and browsers
- React hooks included
- Dual-stack IPv4/IPv6 support
- No API key required
- Zero dependencies (React is optional)

## Installation

```bash
npm install myip-foo
```

## Quick Start

### Node.js / Browser

```typescript
import { getIP, getIPData, getDualStack } from 'myip-foo';

// Get plain IP
const ip = await getIP();
console.log(ip); // "203.0.113.42"

// Get full data with geolocation
const data = await getIPData();
console.log(data.ip);                    // "203.0.113.42"
console.log(data.location.city);         // "Amsterdam"
console.log(data.location.country);      // "NL"
console.log(data.network.isp);           // "KPN B.V."
console.log(data.connectionType);        // "residential"

// Get both IPv4 and IPv6
const dualStack = await getDualStack();
console.log(dualStack.ipv4); // "203.0.113.42"
console.log(dualStack.ipv6); // "2001:db8::1" or null
```

### React

```tsx
import { useIPData, useDualStack, useIP } from 'myip-foo';

function MyComponent() {
  const { data, loading, error, refetch } = useIPData();

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error}</p>;

  return (
    <div>
      <p>IP: {data?.ip}</p>
      <p>Location: {data?.location.city}, {data?.location.country}</p>
      <p>ISP: {data?.network.isp}</p>
      <button onClick={refetch}>Refresh</button>
    </div>
  );
}
```

## API Reference

### Functions

#### `getIP(): Promise<string>`
Returns your IP address as plain text.

#### `getIPData(): Promise<IPData>`
Returns full IP data including geolocation.

#### `getDualStack(): Promise<DualStackData>`
Returns both IPv4 and IPv6 addresses (uses dedicated endpoints).

#### `getConnectionType(): Promise<ConnectionTypeData>`
Detects if connection is residential, VPN, or datacenter.

#### `getHeaders(): Promise<Record<string, string>>`
Returns all HTTP headers as seen by the server.

#### `getUserAgent(): Promise<string>`
Returns your user agent string.

### React Hooks

#### `useIPData()`
Hook for fetching full IP data.

```typescript
const { data, loading, error, refetch } = useIPData();
```

#### `useDualStack()`
Hook for fetching IPv4 and IPv6.

```typescript
const { data, loading, error, refetch } = useDualStack();
// data.ipv4, data.ipv6
```

#### `useIP()`
Hook for fetching plain IP.

```typescript
const { ip, loading, error, refetch } = useIP();
```

### Types

```typescript
interface IPData {
  ip: string;
  type: 'IPv4' | 'IPv6';
  hostname?: string;
  connectionType?: 'residential' | 'vpn' | 'datacenter' | 'tor';
  location: {
    country: string;
    city: string;
    region: string;
    postalCode: string;
    timezone: string;
    latitude: string;
    longitude: string;
  };
  network: {
    asn: number;
    isp: string;
  };
  cloudflare: {
    colo: string;
    ray: string;
  };
}

interface DualStackData {
  ipv4: string | null;
  ipv6: string | null;
}

interface ConnectionTypeData {
  ip: string;
  type: 'residential' | 'vpn' | 'datacenter' | 'unknown';
}
```

## Dual-Stack Endpoints

The `getDualStack()` function uses dedicated subdomains with direct DNS records:

- `ipv4.myip.foo/ip` - Returns IPv4 only (A record)
- `ipv6.myip.foo/ip` - Returns IPv6 only (AAAA record)

This bypasses Cloudflare's dual-stack routing for accurate results.

## Privacy

myip.foo does not log IP addresses or use cookies. See [Privacy Policy](https://myip.foo/privacy.html).

## Links

- **Website:** [myip.foo](https://myip.foo)
- **API Docs:** [myip.foo/api-docs](https://myip.foo/api-docs.html)
- **GitHub:** [myip-packages](https://github.com/virtualox/myip-packages)

## License

MIT License - see [LICENSE](../../LICENSE) for details.
