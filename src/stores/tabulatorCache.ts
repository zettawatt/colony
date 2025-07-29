import { writable } from 'svelte/store';

export type CachedTabulatorInstance = {
  instance: any;
  data: any[];
  columns: any[];
  scrollPosition: { x: number; y: number };
  lastUsed: number;
  isValid: boolean;
};

// Cache for tabulator instances
const tabulatorCache = new Map<string, CachedTabulatorInstance>();

// Store for tracking active cache keys
const { subscribe, set, update } = writable<string[]>([]);

// Cache expiry time (5 minutes)
const CACHE_EXPIRY_MS = 5 * 60 * 1000;

export const tabulatorCacheStore = {
  subscribe,

  // Store a tabulator instance in cache
  cacheInstance: (key: string, instance: any, data: any[], columns: any[], scrollPosition: { x: number; y: number }) => {
    const cachedInstance: CachedTabulatorInstance = {
      instance,
      data: [...data], // Clone the data
      columns: [...columns], // Clone the columns
      scrollPosition: { ...scrollPosition },
      lastUsed: Date.now(),
      isValid: true
    };

    tabulatorCache.set(key, cachedInstance);
    
    update(keys => {
      if (!keys.includes(key)) {
        return [...keys, key];
      }
      return keys;
    });

    console.log(`📦 Cached tabulator instance for key: ${key}`);
  },

  // Retrieve a cached tabulator instance
  getCachedInstance: (key: string): CachedTabulatorInstance | null => {
    const cached = tabulatorCache.get(key);
    
    if (!cached) {
      return null;
    }

    // Check if cache is expired
    const now = Date.now();
    if (now - cached.lastUsed > CACHE_EXPIRY_MS) {
      console.log(`⏰ Cache expired for key: ${key}`);
      tabulatorCache.delete(key);
      update(keys => keys.filter(k => k !== key));
      return null;
    }

    // Update last used time
    cached.lastUsed = now;
    console.log(`✅ Retrieved cached tabulator instance for key: ${key}`);
    return cached;
  },

  // Check if a cached instance exists and is valid
  hasCachedInstance: (key: string): boolean => {
    const cached = tabulatorCache.get(key);
    if (!cached) return false;

    const now = Date.now();
    if (now - cached.lastUsed > CACHE_EXPIRY_MS) {
      tabulatorCache.delete(key);
      update(keys => keys.filter(k => k !== key));
      return false;
    }

    return cached.isValid;
  },

  // Invalidate a cached instance
  invalidateCache: (key: string) => {
    const cached = tabulatorCache.get(key);
    if (cached) {
      cached.isValid = false;
      // Don't destroy immediately, just mark as invalid
      console.log(`❌ Invalidated cache for key: ${key}`);
    }
  },

  // Clear all cached instances
  clearAll: () => {
    for (const [key, cached] of tabulatorCache.entries()) {
      if (cached.instance && typeof cached.instance.destroy === 'function') {
        try {
          cached.instance.destroy();
        } catch (error) {
          console.warn(`Failed to destroy cached tabulator instance for key: ${key}`, error);
        }
      }
    }
    tabulatorCache.clear();
    set([]);
    console.log('🧹 Cleared all tabulator cache');
  },

  // Clean up expired entries
  cleanup: () => {
    const now = Date.now();
    const expiredKeys: string[] = [];

    for (const [key, cached] of tabulatorCache.entries()) {
      if (now - cached.lastUsed > CACHE_EXPIRY_MS) {
        expiredKeys.push(key);
        if (cached.instance && typeof cached.instance.destroy === 'function') {
          try {
            cached.instance.destroy();
          } catch (error) {
            console.warn(`Failed to destroy expired tabulator instance for key: ${key}`, error);
          }
        }
      }
    }

    expiredKeys.forEach(key => tabulatorCache.delete(key));
    
    if (expiredKeys.length > 0) {
      update(keys => keys.filter(k => !expiredKeys.includes(k)));
      console.log(`🧹 Cleaned up ${expiredKeys.length} expired cache entries`);
    }
  },

  // Update scroll position for cached instance
  updateScrollPosition: (key: string, scrollPosition: { x: number; y: number }) => {
    const cached = tabulatorCache.get(key);
    if (cached) {
      cached.scrollPosition = { ...scrollPosition };
      cached.lastUsed = Date.now();
    }
  },

  // Update data for cached instance
  updateData: (key: string, data: any[]) => {
    const cached = tabulatorCache.get(key);
    if (cached) {
      cached.data = [...data];
      cached.lastUsed = Date.now();
    }
  }
};

// Cleanup expired entries every minute
if (typeof window !== 'undefined') {
  setInterval(() => {
    tabulatorCacheStore.cleanup();
  }, 60000);
}
