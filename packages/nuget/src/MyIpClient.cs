using System.Text.Json;

namespace MyIpFoo;

/// <summary>
/// Client for the myip.foo API.
/// </summary>
public class MyIpClient : IDisposable
{
    private readonly HttpClient _httpClient;
    private readonly bool _disposeClient;

    private const string BaseUrl = "https://myip.foo";
    private const string IPv4Url = "https://ipv4.myip.foo";
    private const string IPv6Url = "https://ipv6.myip.foo";

    /// <summary>
    /// Creates a new MyIpClient with a default HttpClient.
    /// </summary>
    public MyIpClient()
    {
        _httpClient = new HttpClient();
        _httpClient.DefaultRequestHeaders.UserAgent.ParseAdd("MyIpFoo/1.0.0");
        _disposeClient = true;
    }

    /// <summary>
    /// Creates a new MyIpClient with a provided HttpClient.
    /// </summary>
    /// <param name="httpClient">The HttpClient to use for requests.</param>
    public MyIpClient(HttpClient httpClient)
    {
        _httpClient = httpClient;
        _disposeClient = false;
    }

    /// <summary>
    /// Gets your IP address as plain text.
    /// </summary>
    public async Task<string> GetIpAsync(CancellationToken cancellationToken = default)
    {
        var response = await _httpClient.GetStringAsync($"{BaseUrl}/plain", cancellationToken);
        return response.Trim();
    }

    /// <summary>
    /// Gets full IP data including geolocation.
    /// </summary>
    public async Task<IpData> GetIpDataAsync(CancellationToken cancellationToken = default)
    {
        var response = await _httpClient.GetStringAsync($"{BaseUrl}/api", cancellationToken);
        return JsonSerializer.Deserialize<IpData>(response) ?? new IpData();
    }

    /// <summary>
    /// Gets both IPv4 and IPv6 addresses.
    /// </summary>
    /// <param name="timeout">Timeout for each request in seconds (default: 5).</param>
    public async Task<DualStackData> GetDualStackAsync(int timeout = 5, CancellationToken cancellationToken = default)
    {
        string? ipv4 = null;
        string? ipv6 = null;

        using var cts = CancellationTokenSource.CreateLinkedTokenSource(cancellationToken);
        cts.CancelAfter(TimeSpan.FromSeconds(timeout));

        try
        {
            var response = await _httpClient.GetStringAsync($"{IPv4Url}/ip", cts.Token);
            var data = JsonSerializer.Deserialize<JsonElement>(response);
            if (data.TryGetProperty("ip", out var ip))
            {
                ipv4 = ip.GetString();
            }
        }
        catch (Exception)
        {
            // IPv4 not available
        }

        try
        {
            var response = await _httpClient.GetStringAsync($"{IPv6Url}/ip", cts.Token);
            var data = JsonSerializer.Deserialize<JsonElement>(response);
            if (data.TryGetProperty("ip", out var ip))
            {
                ipv6 = ip.GetString();
            }
        }
        catch (Exception)
        {
            // IPv6 not available
        }

        return new DualStackData { IPv4 = ipv4, IPv6 = ipv6 };
    }

    /// <summary>
    /// Gets connection type (residential, vpn, datacenter).
    /// </summary>
    public async Task<ConnectionTypeData> GetConnectionTypeAsync(CancellationToken cancellationToken = default)
    {
        var response = await _httpClient.GetStringAsync($"{BaseUrl}/api/connection-type", cancellationToken);
        return JsonSerializer.Deserialize<ConnectionTypeData>(response) ?? new ConnectionTypeData();
    }

    /// <summary>
    /// Gets all HTTP headers as seen by the server.
    /// </summary>
    public async Task<Dictionary<string, string>> GetHeadersAsync(CancellationToken cancellationToken = default)
    {
        var response = await _httpClient.GetStringAsync($"{BaseUrl}/headers", cancellationToken);
        var data = JsonSerializer.Deserialize<JsonElement>(response);
        var headers = new Dictionary<string, string>();

        foreach (var property in data.EnumerateObject())
        {
            headers[property.Name] = property.Value.GetString() ?? "";
        }

        return headers;
    }

    /// <summary>
    /// Gets your user agent string.
    /// </summary>
    public async Task<string> GetUserAgentAsync(CancellationToken cancellationToken = default)
    {
        var response = await _httpClient.GetStringAsync($"{BaseUrl}/user-agent", cancellationToken);
        var data = JsonSerializer.Deserialize<JsonElement>(response);
        return data.TryGetProperty("userAgent", out var ua) ? ua.GetString() ?? "" : "";
    }

    public void Dispose()
    {
        if (_disposeClient)
        {
            _httpClient.Dispose();
        }
    }
}
