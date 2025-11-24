using System.Text.Json.Serialization;

namespace MyIpFoo;

public record Location
{
    [JsonPropertyName("country")]
    public string Country { get; init; } = "";

    [JsonPropertyName("city")]
    public string City { get; init; } = "";

    [JsonPropertyName("region")]
    public string Region { get; init; } = "";

    [JsonPropertyName("postalCode")]
    public string PostalCode { get; init; } = "";

    [JsonPropertyName("timezone")]
    public string Timezone { get; init; } = "";

    [JsonPropertyName("latitude")]
    public string Latitude { get; init; } = "";

    [JsonPropertyName("longitude")]
    public string Longitude { get; init; } = "";
}

public record Network
{
    [JsonPropertyName("asn")]
    public int Asn { get; init; }

    [JsonPropertyName("isp")]
    public string Isp { get; init; } = "";
}

public record CloudflareInfo
{
    [JsonPropertyName("colo")]
    public string Colo { get; init; } = "";

    [JsonPropertyName("ray")]
    public string Ray { get; init; } = "";
}

public record IpData
{
    [JsonPropertyName("ip")]
    public string Ip { get; init; } = "";

    [JsonPropertyName("type")]
    public string Type { get; init; } = "";

    [JsonPropertyName("hostname")]
    public string? Hostname { get; init; }

    [JsonPropertyName("connectionType")]
    public string? ConnectionType { get; init; }

    [JsonPropertyName("location")]
    public Location Location { get; init; } = new();

    [JsonPropertyName("network")]
    public Network Network { get; init; } = new();

    [JsonPropertyName("cloudflare")]
    public CloudflareInfo Cloudflare { get; init; } = new();
}

public record DualStackData
{
    public string? IPv4 { get; init; }
    public string? IPv6 { get; init; }
}

public record ConnectionTypeData
{
    [JsonPropertyName("ip")]
    public string Ip { get; init; } = "";

    [JsonPropertyName("type")]
    public string Type { get; init; } = "";
}
