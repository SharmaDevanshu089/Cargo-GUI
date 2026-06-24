<script>
  import { currentWindow, setWindow } from './navigation.js';
  import { openUrl } from '@tauri-apps/plugin-opener';

  // Issue and feedback links
  const ISSUE_URL = 'https://github.com/SharmaDevanshu089/Cargo-GUI/issues';
  const FEEDBACK_URL = 'https://github.com/SharmaDevanshu089/Cargo-GUI/issues/new?title=Feedback';

  /**
   * Safely opens an external link in the system default browser.
   * Falls back to window.open if not running inside Tauri.
   * @param {string} url
   */
  async function handleOpenLink(url) {
    try {
      if (typeof window !== 'undefined' && window.__TAURI_INTERNALS__) {
        await openUrl(url);
      } else {
        window.open(url, '_blank');
      }
    } catch (error) {
      console.warn('[Sidebar] Falling back to window.open', error);
      if (typeof window !== 'undefined') {
        window.open(url, '_blank');
      }
    }
  }
</script>

<aside class="sidebar">
  <!-- Top: Branding & Logo (with Tauri window drag region) -->
  <div class="sidebar__top" data-tauri-drag-region>
    <h1 class="sidebar__title" data-tauri-drag-region>
      <svg class="sidebar__title-logo" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
        <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
        <polyline points="3.27 6.96 12 12.01 20.73 6.96" />
        <line x1="12" y1="22.08" x2="12" y2="12" />
      </svg>
      Cargo GUI
    </h1>
    <div class="sidebar__subtitle" data-tauri-drag-region>Package Manager v0.1.0</div>
  </div>

  <!-- Middle: Navigation items -->
  <nav class="sidebar__nav">
    
    <!-- Action Button: Create Project (Highlighted at the top) -->
    <button
      class="sidebar__btn sidebar__btn--action { $currentWindow === 'creator' ? 'sidebar__btn--action-active' : '' }"
      onclick={() => setWindow('creator')}
      aria-label="Create Project View"
    >
      <span class="sidebar__icon">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <line x1="12" y1="5" x2="12" y2="19" />
          <line x1="5" y1="12" x2="19" y2="12" />
        </svg>
      </span>
      Create Project
    </button>

    <!-- Cohesive layout separator -->
    <div class="sidebar__nav-separator"></div>

    <!-- Standard navigation items -->
    <button
      class="sidebar__btn { $currentWindow === 'projects' ? 'sidebar__btn--active' : '' }"
      onclick={() => setWindow('projects')}
      aria-label="Projects View"
    >
      <span class="sidebar__icon">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" />
        </svg>
      </span>
      Projects
    </button>

    <button
      class="sidebar__btn { $currentWindow === 'dependencies' ? 'sidebar__btn--active' : '' }"
      onclick={() => setWindow('dependencies')}
      aria-label="Dependencies View"
    >
      <span class="sidebar__icon">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="16.5 9.4 7.5 4.21 7.5 14.17 16.5 19.36 16.5 9.4" />
          <path d="M7.5 4.21L16.5 9.4m-9 4.77L16.5 19.36m-9-5.19v-4.77m0 4.77l-4.5-2.6v-4.77l4.5 2.6" />
        </svg>
      </span>
      Dependencies
    </button>
  </nav>

  <!-- Bottom: Settings & Links -->
  <div class="sidebar__bottom">
    <button
      class="sidebar__btn { $currentWindow === 'settings' ? 'sidebar__btn--active' : '' }"
      onclick={() => setWindow('settings')}
      aria-label="Settings View"
    >
      <span class="sidebar__icon">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="3" />
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" />
        </svg>
      </span>
      Settings
    </button>

    <div class="sidebar__links">
      <button class="sidebar__link" onclick={() => handleOpenLink(ISSUE_URL)}>Issue</button>
      <span class="sidebar__separator">•</span>
      <button class="sidebar__link" onclick={() => handleOpenLink(FEEDBACK_URL)}>Feedback</button>
    </div>
  </div>
</aside>

<style>
  @import "./sidebar.css";
</style>
