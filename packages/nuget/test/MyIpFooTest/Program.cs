using MyIpFoo;

Console.WriteLine("🧪 Testing MyIpFoo NuGet package...\n");

try
{
    using var client = new MyIpClient();

    // Test GetIpAsync
    Console.WriteLine("1. GetIpAsync()");
    var ip = await client.GetIpAsync();
    Console.WriteLine($"   ✅ IP: {ip}\n");

    // Test GetIpDataAsync
    Console.WriteLine("2. GetIpDataAsync()");
    var data = await client.GetIpDataAsync();
    Console.WriteLine($"   ✅ IP: {data.Ip}");
    Console.WriteLine($"   ✅ Type: {data.Type}");
    Console.WriteLine($"   ✅ City: {data.Location.City}");
    Console.WriteLine($"   ✅ Country: {data.Location.Country}");
    Console.WriteLine($"   ✅ ISP: {data.Network.Isp}\n");

    // Test GetDualStackAsync
    Console.WriteLine("3. GetDualStackAsync()");
    var dual = await client.GetDualStackAsync();
    var ipv4Status = dual.IPv4 != null ? "✅" : "❌";
    var ipv6Status = dual.IPv6 != null ? "✅" : "⚠️";
    Console.WriteLine($"   {ipv4Status} IPv4: {dual.IPv4 ?? "not available"}");
    Console.WriteLine($"   {ipv6Status} IPv6: {dual.IPv6 ?? "not available"}\n");

    // Test GetConnectionTypeAsync
    Console.WriteLine("4. GetConnectionTypeAsync()");
    var conn = await client.GetConnectionTypeAsync();
    Console.WriteLine($"   ✅ Type: {conn.Type}");
    Console.WriteLine($"   ✅ IP: {conn.Ip}\n");

    // Test GetHeadersAsync
    Console.WriteLine("5. GetHeadersAsync()");
    var headers = await client.GetHeadersAsync();
    Console.WriteLine($"   ✅ Got {headers.Count} headers\n");

    // Test GetUserAgentAsync
    Console.WriteLine("6. GetUserAgentAsync()");
    var ua = await client.GetUserAgentAsync();
    var uaDisplay = ua.Length > 50 ? ua.Substring(0, 50) + "..." : ua;
    Console.WriteLine($"   ✅ User-Agent: {uaDisplay}\n");

    Console.WriteLine("🎉 All tests passed!");
}
catch (Exception ex)
{
    Console.WriteLine($"❌ Test failed: {ex.Message}");
    Console.WriteLine(ex.StackTrace);
    Environment.Exit(1);
}
