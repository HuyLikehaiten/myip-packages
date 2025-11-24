import { useState, useEffect, useCallback } from 'react';
import { getIP, getIPData, getDualStack } from './client';
import type { IPData, DualStackData } from './types';

interface UseIPDataReturn {
  data: IPData | null;
  loading: boolean;
  error: string | null;
  refetch: () => void;
}

interface UseDualStackReturn {
  data: DualStackData | null;
  loading: boolean;
  error: string | null;
  refetch: () => void;
}

interface UseIPReturn {
  ip: string | null;
  loading: boolean;
  error: string | null;
  refetch: () => void;
}

/**
 * React hook to fetch full IP data including geolocation
 */
export function useIPData(): UseIPDataReturn {
  const [data, setData] = useState<IPData | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const fetchData = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const result = await getIPData();
      setData(result);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch IP data');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchData();
  }, [fetchData]);

  return { data, loading, error, refetch: fetchData };
}

/**
 * React hook to fetch both IPv4 and IPv6 addresses
 */
export function useDualStack(): UseDualStackReturn {
  const [data, setData] = useState<DualStackData | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const fetchData = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const result = await getDualStack();
      setData(result);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch dual-stack data');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchData();
  }, [fetchData]);

  return { data, loading, error, refetch: fetchData };
}

/**
 * React hook to fetch plain IP address
 */
export function useIP(): UseIPReturn {
  const [ip, setIP] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const fetchData = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const result = await getIP();
      setIP(result);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch IP');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchData();
  }, [fetchData]);

  return { ip, loading, error, refetch: fetchData };
}
