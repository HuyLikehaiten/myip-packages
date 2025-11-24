#!/usr/bin/env python3
"""Functional test for myip-foo Python package"""

import sys
sys.path.insert(0, 'src')

from myip_foo import (
    get_ip,
    get_ip_data,
    get_dual_stack,
    get_connection_type,
    get_headers,
    get_user_agent
)

def main():
    print("🧪 Testing myip-foo Python package...\n")

    try:
        # Test get_ip
        print("1. get_ip()")
        ip = get_ip()
        print(f"   ✅ IP: {ip}\n")

        # Test get_ip_data
        print("2. get_ip_data()")
        data = get_ip_data()
        print(f"   ✅ IP: {data['ip']}")
        print(f"   ✅ Type: {data['type']}")
        print(f"   ✅ City: {data['location']['city']}")
        print(f"   ✅ Country: {data['location']['country']}")
        print(f"   ✅ ISP: {data['network']['isp']}\n")

        # Test get_dual_stack
        print("3. get_dual_stack()")
        dual = get_dual_stack()
        ipv4_status = '✅' if dual['ipv4'] else '❌'
        ipv6_status = '✅' if dual['ipv6'] else '⚠️'
        print(f"   {ipv4_status} IPv4: {dual['ipv4'] or 'not available'}")
        print(f"   {ipv6_status} IPv6: {dual['ipv6'] or 'not available'}\n")

        # Test get_connection_type
        print("4. get_connection_type()")
        conn = get_connection_type()
        print(f"   ✅ Type: {conn['type']}")
        print(f"   ✅ IP: {conn['ip']}\n")

        # Test get_headers
        print("5. get_headers()")
        headers = get_headers()
        print(f"   ✅ Got {len(headers)} headers\n")

        # Test get_user_agent
        print("6. get_user_agent()")
        ua = get_user_agent()
        print(f"   ✅ User-Agent: {ua[:50]}...\n")

        print("🎉 All tests passed!")

    except Exception as e:
        print(f"❌ Test failed: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)

if __name__ == "__main__":
    main()
