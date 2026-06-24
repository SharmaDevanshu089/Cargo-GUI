import { writable } from 'svelte/store';

// Svelte store for the currently active window
// Possible values: 'projects', 'dependencies', 'creator', 'settings'
export const currentWindow = writable('projects');

/**
 * Updates the active window.
 * @param {string} windowName
 */
export function setWindow(windowName) {
  if (['projects', 'dependencies', 'creator', 'settings'].includes(windowName)) {
    currentWindow.set(windowName);
  } else {
    console.warn(`[Navigation] Attempted to navigate to unknown window: ${windowName}`);
  }
}
