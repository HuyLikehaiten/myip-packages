# MyIpFoo

Official .NET client for [myip.foo](https://myip.foo) - a free, privacy-focused IP lookup API.

[![NuGet version](https://img.shields.io/nuget/v/MyIpFoo.svg)](https://www.nuget.org/packages/MyIpFoo/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- Async/await support
- Full model types with records
- Dual-stack IPv4/IPv6 support
- Connection type detection
- No API key required
- .NET 6.0+ and .NET 8.0+

## Installation

```bash
dotnet add package MyIpFoo
```

Or via Package Manager:

```powershell
Install-Package MyIpFoo
```

## Quick Start

```csharp
using MyIpFoo;

// Create client (uses default HttpClient)
using var client = new MyIpClient();

// Get plain IP
var ip = await client.GetIpAsync();
Console.WriteLine(ip); // "203.0.113.42"

// Get full data with geolocation
var data = await client.GetIpDataAsync();
Console.WriteLine(data.Ip);                  // "203.0.113.42"
Console.WriteLine(data.Location.City);       // "Amsterdam"
Console.WriteLine(data.Location.Country);    // "NL"
Console.WriteLine(data.Network.Isp);         // "KPN B.V."
Console.WriteLine(data.ConnectionType);      // "residential"

// Get both IPv4 and IPv6
var dualStack = await client.GetDualStackAsync();
Console.WriteLine(dualStack.IPv4); // "203.0.113.42"
Console.WriteLine(dualStack.IPv6); // "2001:db8::1" or null
```

## Dependency Injection

Use with `IHttpClientFactory`:

```csharp
// In Startup.cs or Program.cs
services.AddHttpClient<MyIpClient>();

// In your service
public class MyService
{
    private readonly MyIpClient _myIpClient;

    public MyService(MyIpClient myIpClient)
    {
        _myIpClient = myIpClient;
    }

    public async Task<string> GetCurrentIp()
    {
        return await _myIpClient.GetIpAsync();
    }
}
```

## API Reference

### Methods

#### `GetIpAsync() -> Task<string>`
Returns your IP address as plain text.

#### `GetIpDataAsync() -> Task<IpData>`
Returns full IP data including geolocation.

#### `GetDualStackAsync(int timeout = 5) -> Task<DualStackData>`
Returns both IPv4 and IPv6 addresses.

#### `GetConnectionTypeAsync() -> Task<ConnectionTypeData>`
Detects if connection is residential, VPN, or datacenter.

#### `GetHeadersAsync() -> Task<Dictionary<string, string>>`
Returns all HTTP headers as seen by the server.

#### `GetUserAgentAsync() -> Task<string>`
Returns your user agent string.

### Models

```csharp
public record IpData
{
    public string Ip { get; init; }
    public string Type { get; init; }  // "IPv4" or "IPv6"
    public string? Hostname { get; init; }
    public string? ConnectionType { get; init; }  // "residential", "vpn", "datacenter", "tor"
    public Location Location { get; init; }
    public Network Network { get; init; }
    public CloudflareInfo Cloudflare { get; init; }
}

public record DualStackData
{
    public string? IPv4 { get; init; }
    public string? IPv6 { get; init; }
}

public record ConnectionTypeData
{
    public string Ip { get; init; }
    public string Type { get; init; }  // "residential", "vpn", "datacenter", "unknown"
}
```

## Dual-Stack Endpoints

The `GetDualStackAsync()` method uses dedicated subdomains:

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
