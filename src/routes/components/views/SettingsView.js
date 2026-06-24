import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

// ==========================================
// 📊 State Stores (Svelte Writable Stores)
// ==========================================

export const scanDirectory = writable('');
export const newProjectDirectory = writable('');

// Dropdown selection types: 'drive', 'home', 'documents', 'other'
export const scanDirectoryType = writable('home');
// Dropdown selection types: 'home', 'documents', 'other'
export const newProjectDirectoryType = writable('documents');

// Toolchain versions
export const cargoVersion = writable('Checking...');
export const rustcVersion = writable('Checking...');

// General Preferences
export const autoScan = writable(true);
export const theme = writable('cat-mocha');
export const uiScale = writable(100);
export const saveNotification = writable(false);

// Cache for system paths resolved from Rust
let systemPaths = {
    home: '',
    documents: '',
    drive: ''
};

// Previous selection holders to support cancel/revert on folder pickers
let prevScanType = 'home';
let prevNewProjType = 'documents';

// Subscribe to track previous selections (excluding 'other')
scanDirectoryType.subscribe(value => {
    if (value !== 'other') {
        prevScanType = value;
    }
});

newProjectDirectoryType.subscribe(value => {
    if (value !== 'other') {
        prevNewProjType = value;
    }
});

// ==========================================
// ⚙️ Core Logic Functions
// ==========================================

/**
 * Loads all settings, system paths, and toolchain info from the Rust backend.
 * Automatically resolves the dropdown type based on the loaded path strings.
 */
export async function loadSettings() {
    try {
        // 1. Resolve system paths (Home, Documents, Drive)
        systemPaths = await invoke('get_system_paths');
        
        // 2. Load the AppData configuration file (app.json)
        const config = await invoke('get_app_config');
        scanDirectory.set(config.scanDirectory);
        newProjectDirectory.set(config.newProjectDirectory);

        // 3. Resolve the Scan Directory dropdown type
        const scanPath = config.scanDirectory;
        if (scanPath === systemPaths.drive) {
            scanDirectoryType.set('drive');
        } else if (scanPath === systemPaths.home) {
            scanDirectoryType.set('home');
        } else if (scanPath === systemPaths.documents) {
            scanDirectoryType.set('documents');
        } else {
            scanDirectoryType.set('other');
        }

        // 4. Resolve the New Project Directory dropdown type
        const newProjPath = config.newProjectDirectory;
        if (newProjPath === systemPaths.home) {
            newProjectDirectoryType.set('home');
        } else if (newProjPath === systemPaths.documents) {
            newProjectDirectoryType.set('documents');
        } else {
            newProjectDirectoryType.set('other');
        }

        // 5. Load Rust and Cargo compiler toolchain versions
        const info = await invoke('get_toolchain_info');
        cargoVersion.set(info.cargoVersion);
        rustcVersion.set(info.rustcVersion);
        
    } catch (error) {
        console.error('[Settings Logic] Failed to load settings from backend:', error);
    }
}

/**
 * Serializes and saves the active settings back to app.json in the AppData directory.
 * Translates dropdown types ('home', 'documents', etc.) back into absolute paths.
 */
export async function saveSettings() {
    // Resolve the scan directory path based on the selected dropdown type
    const sType = get(scanDirectoryType);
    let resolvedScanPath = '';
    if (sType === 'drive') {
        resolvedScanPath = systemPaths.drive;
    } else if (sType === 'home') {
        resolvedScanPath = systemPaths.home;
    } else if (sType === 'documents') {
        resolvedScanPath = systemPaths.documents;
    } else {
        resolvedScanPath = get(scanDirectory);
    }

    // Resolve the new project directory path based on the selected dropdown type
    const pType = get(newProjectDirectoryType);
    let resolvedProjPath = '';
    if (pType === 'home') {
        resolvedProjPath = systemPaths.home;
    } else if (pType === 'documents') {
        resolvedProjPath = systemPaths.documents;
    } else {
        resolvedProjPath = get(newProjectDirectory);
    }

    // Save configuration via Tauri command
    try {
        await invoke('save_app_config', {
            config: {
                scanDirectory: resolvedScanPath,
                newProjectDirectory: resolvedProjPath
            }
        });

        // Trigger a visual confirmation toast briefly
        saveNotification.set(true);
        setTimeout(() => {
            saveNotification.set(false);
        }, 1500);
    } catch (error) {
        console.error('[Settings Logic] Failed to save settings to backend:', error);
    }
}

/**
 * Handles dropdown changes for the Scan Directory.
 * Opens the native folder picker if "Other..." is selected.
 * Reverts back to the previous type if the picker is canceled.
 * @param {string} type 
 */
export async function changeScanDirectoryType(type) {
    if (type === 'other') {
        // Trigger the native OS folder picker using our rfd crate
        const selected = await invoke('pick_folder');
        if (selected) {
            scanDirectory.set(selected);
            scanDirectoryType.set('other');
            await saveSettings();
        } else {
            // Revert back to the previous selection type if user cancelled
            scanDirectoryType.set(prevScanType);
        }
    } else {
        scanDirectoryType.set(type);
        await saveSettings();
    }
}

/**
 * Handles dropdown changes for the New Project Directory.
 * Opens the native folder picker if "Other..." is selected.
 * Reverts back to the previous type if the picker is canceled.
 * @param {string} type 
 */
export async function changeNewProjectDirectoryType(type) {
    if (type === 'other') {
        // Trigger the native OS folder picker using our rfd crate
        const selected = await invoke('pick_folder');
        if (selected) {
            newProjectDirectory.set(selected);
            newProjectDirectoryType.set('other');
            await saveSettings();
        } else {
            // Revert back to the previous selection type if user cancelled
            newProjectDirectoryType.set(prevNewProjType);
        }
    } else {
        newProjectDirectoryType.set(type);
        await saveSettings();
    }
}

/**
 * Triggers the folder picker to change the custom scan path.
 */
export async function browseCustomScanDirectory() {
    const selected = await invoke('pick_folder');
    if (selected) {
        scanDirectory.set(selected);
        await saveSettings();
    }
}

/**
 * Triggers the folder picker to change the custom project path.
 */
export async function browseCustomNewProjectDirectory() {
    const selected = await invoke('pick_folder');
    if (selected) {
        newProjectDirectory.set(selected);
        await saveSettings();
    }
}
