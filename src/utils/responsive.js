import { writable } from 'svelte/store';

// Mobile breakpoint - screens smaller than this will be considered mobile
export const MOBILE_BREAKPOINT = 768;

// Reactive store for mobile state
export const isMobile = writable(false);

// Function to check if current screen size is mobile
export function checkIsMobile() {
  if (typeof window !== 'undefined') {
    return window.innerWidth < MOBILE_BREAKPOINT;
  }
  return false;
}

// Initialize mobile detection
export function initMobileDetection() {
  if (typeof window !== 'undefined') {
    // Set initial state
    isMobile.set(checkIsMobile());
    
    // Listen for window resize events
    const handleResize = () => {
      isMobile.set(checkIsMobile());
    };
    
    window.addEventListener('resize', handleResize);
    
    // Return cleanup function
    return () => {
      window.removeEventListener('resize', handleResize);
    };
  }
  return () => {};
}

/**
 * Utility function to get mobile-specific column configurations
 * @param {any[]} allColumns - Array of all available columns
 * @param {string[]} mobileColumnKeys - Array of column keys to show on mobile
 * @returns {any[]} Filtered columns for mobile or all columns for desktop
 */
export function getMobileColumns(allColumns, mobileColumnKeys) {
  if (!checkIsMobile()) {
    return allColumns;
  }

  return allColumns.filter(column =>
    mobileColumnKeys.includes(column.field) ||
    mobileColumnKeys.includes(column.title)
  );
}
