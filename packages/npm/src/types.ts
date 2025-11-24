export interface Location {
  country: string;
  city: string;
  region: string;
  postalCode: string;
  timezone: string;
  latitude: string;
  longitude: string;
}

export interface Network {
  asn: number;
  isp: string;
}

export interface Cloudflare {
  colo: string;
  ray: string;
}

export interface IPData {
  ip: string;
  type: 'IPv4' | 'IPv6';
  hostname?: string;
  connectionType?: 'residential' | 'vpn' | 'datacenter' | 'tor';
  location: Location;
  network: Network;
  cloudflare: Cloudflare;
}

export interface DualStackData {
  ipv4: string | null;
  ipv6: string | null;
}

export interface ConnectionTypeData {
  ip: string;
  type: 'residential' | 'vpn' | 'datacenter' | 'unknown';
}
