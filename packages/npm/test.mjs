// Quick functional test
import { getIP, getIPData, getDualStack, getConnectionType } from './dist/index.mjs';

async function test() {
  console.log('🧪 Testing myip-foo package...\n');

  try {
    // Test getIP
    console.log('1. getIP()');
    const ip = await getIP();
    console.log(`   ✅ IP: ${ip}\n`);

    // Test getIPData
    console.log('2. getIPData()');
    const data = await getIPData();
    console.log(`   ✅ IP: ${data.ip}`);
    console.log(`   ✅ Type: ${data.type}`);
    console.log(`   ✅ City: ${data.location.city}`);
    console.log(`   ✅ Country: ${data.location.country}`);
    console.log(`   ✅ ISP: ${data.network.isp}\n`);

    // Test getDualStack
    console.log('3. getDualStack()');
    const dual = await getDualStack();
    console.log(`   ${dual.ipv4 ? '✅' : '❌'} IPv4: ${dual.ipv4 || 'not available'}`);
    console.log(`   ${dual.ipv6 ? '✅' : '⚠️'} IPv6: ${dual.ipv6 || 'not available'}\n`);

    // Test getConnectionType
    console.log('4. getConnectionType()');
    const conn = await getConnectionType();
    console.log(`   ✅ Type: ${conn.type}`);
    console.log(`   ✅ IP: ${conn.ip}\n`);

    console.log('🎉 All tests passed!');
  } catch (error) {
    console.error('❌ Test failed:', error.message);
    process.exit(1);
  }
}

test();
