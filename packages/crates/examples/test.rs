use myip_foo::{get_connection_type, get_dual_stack, get_headers, get_ip, get_ip_data, get_user_agent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing myip-foo crate...\n");

    // Test get_ip
    println!("1. get_ip()");
    let ip = get_ip().await?;
    println!("   ✅ IP: {}\n", ip);

    // Test get_ip_data
    println!("2. get_ip_data()");
    let data = get_ip_data().await?;
    println!("   ✅ IP: {}", data.ip);
    println!("   ✅ Type: {}", data.ip_type);
    println!("   ✅ City: {}", data.location.city);
    println!("   ✅ Country: {}", data.location.country);
    println!("   ✅ ISP: {}\n", data.network.isp);

    // Test get_dual_stack
    println!("3. get_dual_stack()");
    let dual = get_dual_stack().await?;
    let ipv4_status = if dual.ipv4.is_some() { "✅" } else { "❌" };
    let ipv6_status = if dual.ipv6.is_some() { "✅" } else { "⚠️" };
    println!(
        "   {} IPv4: {}",
        ipv4_status,
        dual.ipv4.as_deref().unwrap_or("not available")
    );
    println!(
        "   {} IPv6: {}\n",
        ipv6_status,
        dual.ipv6.as_deref().unwrap_or("not available")
    );

    // Test get_connection_type
    println!("4. get_connection_type()");
    let conn = get_connection_type().await?;
    println!("   ✅ Type: {}", conn.connection_type);
    println!("   ✅ IP: {}\n", conn.ip);

    // Test get_headers
    println!("5. get_headers()");
    let headers = get_headers().await?;
    println!("   ✅ Got {} headers\n", headers.len());

    // Test get_user_agent
    println!("6. get_user_agent()");
    let ua = get_user_agent().await?;
    let ua_display = if ua.len() > 50 {
        format!("{}...", &ua[..50])
    } else {
        ua.clone()
    };
    println!("   ✅ User-Agent: {}\n", ua_display);

    println!("🎉 All tests passed!");

    Ok(())
}
