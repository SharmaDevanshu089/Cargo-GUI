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
    saveNotification,
    changeScanDirectoryType,
    changeNewProjectDirectoryType,
    browseCustomScanDirectory,
    browseCustomNewProjectDirectory
  } from './SettingsView.js';

  // Load configuration and toolchain info when component mounts
  onMount(() => {
    loadSettings();
  });
</script>

<div class="settings-view">
  <!-- Header -->
  <header class="view-header">
    <div>
      <h2 class="view-title">Settings</h2>
      <p class="view-subtitle">Configure system directories and view toolchain diagnostics.</p>
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

  <!-- Linear Scrollable Layout -->
  <div class="settings-layout">
    
    <!-- Section 1: Directories -->
    <section class="settings-section">
      <h3 class="settings-section__title">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2" />
          <line x1="9" y1="3" x2="9" y2="21" />
        </svg>
        Directories
      </h3>
      
      <div class="settings-section__content">
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
          
          <!-- Conditional Custom Scan Path Picker -->
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

          <!-- Conditional Custom Project Path Picker -->
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

        <!-- Forced Auto-scan Startup Toggle -->
        <div class="toggle-row">
          <div class="toggle-info">
            <span class="toggle-label">Auto-scan on Startup</span>
            <span class="toggle-desc">Scan the default directory automatically when opening the application. (Always Active)</span>
          </div>
          <label class="switch switch--disabled" aria-label="Auto-scan on Startup Toggle (Forced On)">
            <input type="checkbox" checked disabled />
            <span class="slider"></span>
          </label>
        </div>
      </div>
    </section>

    <!-- Divider line -->
    <hr class="settings-divider" />

    <!-- Section 2: System Diagnostics -->
    <section class="settings-section">
      <h3 class="settings-section__title">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10" />
          <line x1="12" y1="16" x2="12" y2="12" />
          <line x1="12" y1="8" x2="12.01" y2="8" />
        </svg>
        System Diagnostics
      </h3>
      
      <div class="settings-section__content">
        <p class="diagnostics-intro">Cargo GUI relies on your local Rust toolchain. Below is your active compiler environment info.</p>
        
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
          </tbody>
        </table>
      </div>
    </section>

  </div>
</div>

<style>
  .settings-view {
    display: flex;
    flex-direction: column;
    gap: 32px;
    height: 100%;
    max-width: 800px;
    margin: 0;
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

  /* Layout list */
  .settings-layout {
    display: flex;
    flex-direction: column;
    gap: 28px;
    padding: 8px 0;
  }

  .settings-divider {
    border: none;
    border-top: 1px solid rgba(49, 50, 68, 0.4);
    margin: 8px 0;
    width: 100%;
    max-width: 540px;
  }

  /* Flat Sections */
  .settings-section {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .settings-section__title {
    font-size: 15px;
    font-weight: 600;
    color: var(--cat-text);
    margin: 0;
    display: flex;
    align-items: center;
    gap: 10px;
    color: #b4befe; /* Lavender accent for section titles */
  }

  .settings-section__content {
    display: flex;
    flex-direction: column;
    gap: 20px;
    padding-left: 26px; /* indent content slightly under title */
  }

  /* Form Elements */
  .form-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
    max-width: 540px;
  }

  .form-label {
    font-size: 13px;
    font-weight: 600;
    color: var(--cat-text);
  }

  .form-input, .form-select {
    padding: 10px 14px;
    background-color: var(--cat-mantle); /* Darker background for contrast against page base */
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
    background-color: var(--cat-mantle); /* Darker background for contrast against page base */
    border: 1px solid var(--cat-surface0);
    padding: 14px 16px;
    border-radius: 8px;
    max-width: 540px;
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

  /* Disabled State (Forced On) */
  .switch--disabled .slider {
    background-color: rgba(180, 190, 254, 0.4); /* Faded lavender */
    cursor: not-allowed;
  }

  .switch--disabled .slider:before {
    transform: translateX(20px);
    background-color: var(--cat-mantle);
  }

  /* Diagnostics Section */
  .diagnostics-intro {
    font-size: 13px;
    color: var(--cat-subtext);
    line-height: 1.5;
    margin: 0;
    max-width: 540px;
  }

  .diagnostics-table {
    width: 100%;
    max-width: 540px;
    border-collapse: collapse;
    margin-top: 6px;
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
    width: 30%;
  }

  .diagnostic-value {
    color: var(--cat-text);
  }

  .font-mono {
    font-family: 'Courier New', Courier, monospace;
    word-break: break-all;
  }

  /* Transitions & Animations */
  .fade-in {
    animation: fadeIn 0.2s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(3px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
