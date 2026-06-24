<script>
  import { onMount } from 'svelte';
  import {
    loadSettings,
    scanDirectory,
    newProjectDirectory,
    scanDirectoryType,
    newProjectDirectoryType,
    cargoVersion,
    rustcVersion,
    autoScan,
    theme,
    uiScale,
    saveNotification,
    changeScanDirectoryType,
    changeNewProjectDirectoryType,
    browseCustomScanDirectory,
    browseCustomNewProjectDirectory
  } from './SettingsView.js';

  // Load backend configurations and toolchain info when component mounts
  onMount(() => {
    loadSettings();
  });
</script>

<div class="view-container">
  <!-- Header -->
  <header class="view-header">
    <div>
      <h2 class="view-title">Settings</h2>
      <p class="view-subtitle">Configure system directories, user interface preferences, and explore toolchain diagnostics.</p>
    </div>

    {#if $saveNotification}
      <div class="save-toast" role="status">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
          <polyline points="20 6 9 17 4 12" />
        </svg>
        Preferences saved
      </div>
    {/if}
  </header>

  <div class="settings-grid">
    <!-- Left Column: Preferences -->
    <div class="settings-column">
      <!-- General Preferences (Directories) -->
      <section class="settings-card">
        <h3 class="settings-card__title">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" />
            <line x1="9" y1="3" x2="9" y2="21" />
          </svg>
          General (Directories)
        </h3>
        
        <div class="settings-card__body">
          <!-- Scan Directory Dropdown -->
          <div class="form-group">
            <label for="scan-dir-type" class="form-label">Projects Scan Directory</label>
            <select 
              id="scan-dir-type" 
              value={$scanDirectoryType} 
              onchange={(e) => changeScanDirectoryType(e.target.value)} 
              class="form-select"
            >
              <option value="drive">(Entire Drive)</option>
              <option value="home">Home Folder</option>
              <option value="documents">Documents Folder</option>
              <option value="other">Other...</option>
            </select>
            <span class="form-help">The primary folder Cargo GUI will scan for crates on startup.</span>
            
            <!-- Conditional Custom Scan Path Input & Browse Button -->
            {#if $scanDirectoryType === 'other'}
              <div class="custom-path-group fade-in">
                <div class="browse-input">
                  <input 
                    type="text" 
                    value={$scanDirectory} 
                    readonly 
                    class="form-input font-mono" 
                    title="Custom Scan Directory"
                  />
                  <button type="button" class="btn-secondary" onclick={browseCustomScanDirectory}>Browse</button>
                </div>
              </div>
            {/if}
          </div>

          <!-- New Project Directory Dropdown -->
          <div class="form-group">
            <label for="proj-dir-type" class="form-label">Default Project Creation Directory</label>
            <select 
              id="proj-dir-type" 
              value={$newProjectDirectoryType} 
              onchange={(e) => changeNewProjectDirectoryType(e.target.value)} 
              class="form-select"
            >
              <option value="home">Home Folder</option>
              <option value="documents">Documents Folder</option>
              <option value="other">Other...</option>
            </select>
            <span class="form-help">The default folder where new Cargo projects will be initialized.</span>

            <!-- Conditional Custom Project Path Input & Browse Button -->
            {#if $newProjectDirectoryType === 'other'}
              <div class="custom-path-group fade-in">
                <div class="browse-input">
                  <input 
                    type="text" 
                    value={$newProjectDirectory} 
                    readonly 
                    class="form-input font-mono" 
                    title="Custom New Project Directory"
                  />
                  <button type="button" class="btn-secondary" onclick={browseCustomNewProjectDirectory}>Browse</button>
                </div>
              </div>
            {/if}
          </div>

          <!-- Auto-scan Startup Toggle -->
          <div class="toggle-row">
            <div class="toggle-info">
              <span class="toggle-label">Auto-scan on Startup</span>
              <span class="toggle-desc">Scan the default directory automatically when opening the application.</span>
            </div>
            <label class="switch" aria-label="Auto-scan on Startup Toggle">
              <input type="checkbox" bind:checked={$autoScan} />
              <span class="slider"></span>
            </label>
          </div>
        </div>
      </section>

      <!-- Appearance Preferences -->
      <section class="settings-card">
        <h3 class="settings-card__title">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z" />
            <path d="M12 18a6 6 0 1 0 0-12 6 6 0 0 0 0 12z" />
          </svg>
          Appearance
        </h3>

        <div class="settings-card__body">
          <div class="form-group">
            <label for="color-theme" class="form-label">Color Theme</label>
            <select id="color-theme" bind:value={$theme} class="form-select">
              <option value="cat-mocha">Catppuccin Mocha (Default Dark)</option>
              <option value="cat-macchiato">Catppuccin Macchiato</option>
              <option value="cat-latte">Catppuccin Latte (Light)</option>
              <option value="classic-dark">Classic High Contrast Dark</option>
              <option value="system-default">System Default</option>
            </select>
          </div>

          <div class="form-group">
            <div class="slider-header">
              <label for="ui-scale" class="form-label">UI Scale</label>
              <span class="slider-value">{$uiScale}%</span>
            </div>
            <input 
              type="range" 
              id="ui-scale"
              min="80" 
              max="120" 
              step="5" 
              bind:value={$uiScale} 
              class="form-range" 
            />
            <span class="form-help">Adjust application interface size.</span>
          </div>
        </div>
      </section>
    </div>

    <!-- Right Column: System Diagnostics -->
    <div class="settings-column">
      <section class="settings-card settings-card--diagnostics">
        <h3 class="settings-card__title">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10" />
            <line x1="12" y1="16" x2="12" y2="12" />
            <line x1="12" y1="8" x2="12.01" y2="8" />
          </svg>
          System Diagnostics
        </h3>
        
        <div class="settings-card__body">
          <p class="diagnostics-intro">Cargo GUI relies on your local Rust toolchain. Below is your active environment info.</p>
          
          <table class="diagnostics-table">
            <tbody>
              <tr>
                <td class="diagnostic-name">Cargo Version</td>
                <td class="diagnostic-value font-mono">{$cargoVersion}</td>
              </tr>
              <tr>
                <td class="diagnostic-name">Rustc Version</td>
                <td class="diagnostic-value font-mono">{$rustcVersion}</td>
              </tr>
              <tr>
                <td class="diagnostic-name">Tauri Framework</td>
                <td class="diagnostic-value font-mono">v2.0.0 (Win32 + Webview2)</td>
              </tr>
              <tr>
                <td class="diagnostic-name">Frontend System</td>
                <td class="diagnostic-value font-mono">Svelte v5.56.3 + Vite v8.0.16</td>
              </tr>
              <tr>
                <td class="diagnostic-name">Host OS</td>
                <td class="diagnostic-value font-mono">Windows 11 Home (Build 22631)</td>
              </tr>
            </tbody>
          </table>

          <div class="diagnostics-status">
            <span class="status-indicator">All tools connected</span>
          </div>
        </div>
      </section>
    </div>
  </div>
</div>

<style>
  .view-container {
    display: flex;
    flex-direction: column;
    gap: 24px;
    height: 100%;
    max-width: 1200px;
    margin: 0 auto;
  }

  /* Header */
  .view-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    position: relative;
  }

  .view-title {
    font-size: 24px;
    font-weight: 700;
    color: var(--cat-text);
    margin: 0;
  }

  .view-subtitle {
    font-size: 14px;
    color: var(--cat-subtext);
    margin: 6px 0 0 0;
  }

  /* Save Toast */
  .save-toast {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    background-color: rgba(166, 227, 161, 0.1);
    color: #a6e3a1;
    padding: 8px 14px;
    border-radius: 20px;
    font-size: 13px;
    font-weight: 600;
    border: 1px solid rgba(166, 227, 161, 0.2);
    animation: fadeIn 0.2s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(5px); }
    to { opacity: 1; transform: translateY(0); }
  }

  /* Settings Grid */
  .settings-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 24px;
  }

  @media (min-width: 900px) {
    .settings-grid {
      grid-template-columns: 1.1fr 0.9fr;
    }
  }

  .settings-column {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .settings-card {
    background-color: var(--cat-mantle);
    border: 1px solid var(--cat-surface0);
    border-radius: 12px;
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 18px;
  }

  .settings-card__title {
    font-size: 16px;
    font-weight: 600;
    color: var(--cat-text);
    margin: 0;
    display: flex;
    align-items: center;
    gap: 10px;
    padding-bottom: 12px;
    border-bottom: 1px solid rgba(49, 50, 68, 0.5);
  }

  .settings-card__title svg {
    color: #b4befe;
  }

  .settings-card__body {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  /* Form Elements */
  .form-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .form-label {
    font-size: 13px;
    font-weight: 600;
    color: var(--cat-text);
  }

  .form-input, .form-select {
    padding: 10px 14px;
    background-color: var(--cat-base);
    border: 1px solid var(--cat-surface0);
    border-radius: 8px;
    color: var(--cat-text);
    font-family: inherit;
    font-size: 14px;
    outline: none;
    transition: border-color 0.2s ease;
  }

  .form-input:focus, .form-select:focus {
    border-color: #b4befe;
  }

  .form-select {
    cursor: pointer;
    appearance: none;
    background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='24' height='24' viewBox='0 0 24 24' fill='none' stroke='%23a6adc8' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><polyline points='6 9 12 15 18 9'></polyline></svg>");
    background-repeat: no-repeat;
    background-position: right 14px center;
    background-size: 16px;
    padding-right: 40px;
  }

  .form-help {
    font-size: 11px;
    color: var(--cat-subtext);
  }

  /* Browse Input */
  .browse-input {
    display: flex;
    gap: 10px;
  }

  .browse-input input {
    flex: 1;
  }

  .btn-secondary {
    padding: 10px 16px;
    background-color: var(--cat-surface0);
    color: var(--cat-text);
    border: none;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: background-color 0.2s ease;
  }

  .btn-secondary:hover {
    background-color: #45475a;
  }

  .custom-path-group {
    margin-top: 4px;
  }

  /* Switch Toggle Row */
  .toggle-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background-color: var(--cat-base);
    border: 1px solid var(--cat-surface0);
    padding: 14px 16px;
    border-radius: 8px;
  }

  .toggle-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
    max-width: 80%;
  }

  .toggle-label {
    font-size: 13px;
    font-weight: 600;
    color: var(--cat-text);
  }

  .toggle-desc {
    font-size: 11px;
    color: var(--cat-subtext);
    line-height: 1.4;
  }

  /* Switch Slider */
  .switch {
    position: relative;
    display: inline-block;
    width: 44px;
    height: 24px;
    flex-shrink: 0;
  }

  .switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: var(--cat-surface0);
    transition: 0.3s;
    border-radius: 24px;
  }

  .slider:before {
    position: absolute;
    content: "";
    height: 18px;
    width: 18px;
    left: 3px;
    bottom: 3px;
    background-color: var(--cat-text);
    transition: 0.3s;
    border-radius: 50%;
  }

  input:checked + .slider {
    background-color: #b4befe;
  }

  input:checked + .slider:before {
    transform: translateX(20px);
    background-color: #11111b;
  }

  /* Range Slider */
  .slider-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .slider-value {
    font-size: 12px;
    font-weight: 600;
    color: #b4befe;
  }

  .form-range {
    -webkit-appearance: none;
    width: 100%;
    height: 6px;
    border-radius: 3px;
    background: var(--cat-base);
    outline: none;
    margin: 8px 0;
  }

  .form-range::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: #b4befe;
    cursor: pointer;
    transition: transform 0.1s ease;
  }

  .form-range::-webkit-slider-thumb:hover {
    transform: scale(1.2);
  }

  /* Diagnostics Column */
  .diagnostics-intro {
    font-size: 13px;
    color: var(--cat-subtext);
    line-height: 1.5;
    margin: 0;
  }

  .diagnostics-table {
    width: 100%;
    border-collapse: collapse;
    margin: 10px 0;
  }

  .diagnostics-table td {
    padding: 12px 0;
    border-bottom: 1px solid rgba(49, 50, 68, 0.4);
    font-size: 13px;
  }

  .diagnostics-table tr:last-child td {
    border-bottom: none;
  }

  .diagnostic-name {
    color: var(--cat-subtext);
    font-weight: 500;
    width: 35%;
  }

  .diagnostic-value {
    color: var(--cat-text);
  }

  .font-mono {
    font-family: 'Courier New', Courier, monospace;
    word-break: break-all;
  }

  .diagnostics-status {
    padding-top: 14px;
    border-top: 1px solid rgba(49, 50, 68, 0.5);
  }

  .status-indicator {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    color: #a6e3a1;
    font-size: 13px;
    font-weight: 600;
  }

  .status-indicator::before {
    content: "";
    width: 8px;
    height: 8px;
    background-color: #a6e3a1;
    border-radius: 50%;
    box-shadow: 0 0 8px #a6e3a1;
  }

  /* Transitions */
  .fade-in {
    animation: fadeIn 0.2s ease-out;
  }
</style>
