// Core client functions
export {
  getIP,
  getIPData,
  getDualStack,
  getConnectionType,
  getHeaders,
  getUserAgent
} from './client';

// React hooks
export {
  useIPData,
  useDualStack,
  useIP
} from './react';

// Types
export type {
  IPData,
  DualStackData,
  ConnectionTypeData,
  Location,
  Network,
  Cloudflare
} from './types';
