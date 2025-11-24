import type { IPData, DualStackData, ConnectionTypeData } from './types';

const BASE_URL = 'https://myip.foo';
const IPV4_URL = 'https://ipv4.myip.foo';
const IPV6_URL = 'https://ipv6.myip.foo';

/**
 * Get your IP address as plain text
 */
export async function getIP(): Promise<string> {
  const response = await fetch(`${BASE_URL}/plain`);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return (await response.text()).trim();
}

/**
 * Get full IP data including geolocation
 */
export async function getIPData(): Promise<IPData> {
  const response = await fetch(`${BASE_URL}/api`);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return response.json();
}

/**
 * Get both IPv4 and IPv6 addresses
 */
export async function getDualStack(): Promise<DualStackData> {
  const [ipv4Response, ipv6Response] = await Promise.allSettled([
    fetch(`${IPV4_URL}/ip`, { signal: AbortSignal.timeout(5000) }),
    fetch(`${IPV6_URL}/ip`, { signal: AbortSignal.timeout(5000) })
  ]);

  let ipv4: string | null = null;
  let ipv6: string | null = null;

  if (ipv4Response.status === 'fulfilled' && ipv4Response.value.ok) {
    const data = await ipv4Response.value.json();
    ipv4 = data.ip;
  }

  if (ipv6Response.status === 'fulfilled' && ipv6Response.value.ok) {
    const data = await ipv6Response.value.json();
    ipv6 = data.ip;
  }

  return { ipv4, ipv6 };
}

/**
 * Get connection type (residential, vpn, datacenter)
 */
export async function getConnectionType(): Promise<ConnectionTypeData> {
  const response = await fetch(`${BASE_URL}/api/connection-type`);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return response.json();
}

/**
 * Get all HTTP headers as seen by the server
 */
export async function getHeaders(): Promise<Record<string, string>> {
  const response = await fetch(`${BASE_URL}/headers`);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  return response.json();
}

/**
 * Get user agent string
 */
export async function getUserAgent(): Promise<string> {
  const response = await fetch(`${BASE_URL}/user-agent`);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  const data = await response.json();
  return data.userAgent;
}
