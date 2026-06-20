import { getCurrentWindow } from '@tauri-apps/api/window';

/**
 * Minimizes the current Tauri window.
 * @returns {Promise<void>}
 */
export async function minimizeWindow() {
  console.log('[Titlebar] Initiating minimizeWindow');
  try {
    const appWindow = getCurrentWindow();
    await appWindow.minimize();
    console.log('[Titlebar] Window minimized successfully');
  } catch (error) {
    console.error('[Titlebar] Failed to minimize window:', error);
  }
}

/**
 * Toggles maximize/restore state on the current Tauri window.
 * @returns {Promise<boolean>} The new maximized state (true if maximized, false if restored).
 */
export async function toggleMaximizeWindow() {
  console.log('[Titlebar] Initiating toggleMaximizeWindow');
  try {
    const appWindow = getCurrentWindow();
    const isMaximized = await appWindow.isMaximized();
    if (isMaximized) {
      console.log('[Titlebar] Window is currently maximized, unmaximizing...');
      await appWindow.unmaximize();
      return false;
    } else {
      console.log('[Titlebar] Window is currently restored, maximizing...');
      await appWindow.maximize();
      return true;
    }
  } catch (error) {
    console.error('[Titlebar] Failed to toggle window maximization:', error);
    return false;
  }
}

/**
 * Closes the current Tauri window.
 * @returns {Promise<void>}
 */
export async function closeWindow() {
  console.log('[Titlebar] Initiating closeWindow');
  try {
    const appWindow = getCurrentWindow();
    await appWindow.close();
    console.log('[Titlebar] Window close command sent');
  } catch (error) {
    console.error('[Titlebar] Failed to close window:', error);
  }
}
