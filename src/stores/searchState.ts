import { writable } from 'svelte/store';

export type SearchMetrics = {
  itemCount: number;
  searchTime: number;
  hasSearched: boolean;
};

export type SearchStateData = {
  searchInput: string;
  tableSearchResults: any[];
  searchMetrics: SearchMetrics;
  activeRow: any;
  scrollPosition: {
    x: number;
    y: number;
  };
};

// Default search state
const defaultSearchState: SearchStateData = {
  searchInput: "",
  tableSearchResults: [],
  searchMetrics: {
    itemCount: 0,
    searchTime: 0,
    hasSearched: false
  },
  activeRow: {},
  scrollPosition: {
    x: 0,
    y: 0
  }
};

// Create the writable store
const { subscribe, set, update } = writable<SearchStateData>(defaultSearchState);

// Session storage key
const SESSION_STORAGE_KEY = 'colony-search-state';

// Load initial state from session storage (browser session only)
let initialized = false;

function initializeSearchState() {
  if (initialized) return;

  try {
    // Only use session storage in browser environment
    if (typeof window !== 'undefined' && window.sessionStorage) {
      const savedState = sessionStorage.getItem(SESSION_STORAGE_KEY);
      if (savedState) {
        const parsedState = JSON.parse(savedState);
        console.log('Loading saved search state from session:', parsedState);
        set(parsedState);
      } else {
        console.log('No saved search state found in session, using default');
      }
    }
    initialized = true;
  } catch (error) {
    console.error('Failed to load search state from session:', error);
    initialized = true;
  }
}

// Save state to session storage
function saveToSession(state: SearchStateData) {
  try {
    if (typeof window !== 'undefined' && window.sessionStorage) {
      sessionStorage.setItem(SESSION_STORAGE_KEY, JSON.stringify(state));
    }
  } catch (error) {
    console.error('Failed to save search state to session:', error);
  }
}

// Custom store with session persistence
export const searchState = {
  subscribe,

  // Initialize the store (call this when the app starts)
  init: initializeSearchState,

  // Set search input and persist to session
  setSearchInput: (input: string) => {
    update(state => {
      const newState = { ...state, searchInput: input };
      console.log('Saving search input:', input);
      saveToSession(newState);
      return newState;
    });
  },

  // Set search results and persist to session
  setSearchResults: (results: any[]) => {
    update(state => {
      const newState = { ...state, tableSearchResults: results };
      saveToSession(newState);
      return newState;
    });
  },

  // Set search metrics and persist to session
  setSearchMetrics: (metrics: SearchMetrics) => {
    update(state => {
      const newState = { ...state, searchMetrics: metrics };
      saveToSession(newState);
      return newState;
    });
  },

  // Set active row and persist to session
  setActiveRow: (row: any) => {
    update(state => {
      const newState = { ...state, activeRow: row };
      saveToSession(newState);
      return newState;
    });
  },

  // Set scroll position and persist to session
  setScrollPosition: (x: number, y: number) => {
    update(state => {
      const newState = { ...state, scrollPosition: { x, y } };
      saveToSession(newState);
      return newState;
    });
  },

  // Update multiple fields at once
  updateState: (updates: Partial<SearchStateData>) => {
    update(state => {
      const newState = { ...state, ...updates };
      saveToSession(newState);
      return newState;
    });
  },

  // Clear all search state
  clear: () => {
    set(defaultSearchState);
    saveToSession(defaultSearchState);
  },

  // Reset to default state
  reset: () => {
    set(defaultSearchState);
    saveToSession(defaultSearchState);
  }
};
