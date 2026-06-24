<script>
  import { minimizeWindow, toggleMaximizeWindow, closeWindow } from './titlebar.js';

  let isMaximized = $state(false);

  async function handleMinimize() {
    await minimizeWindow();
  }

  async function handleMaximizeRestore() {
    isMaximized = await toggleMaximizeWindow();
  }

  async function handleClose() {
    await closeWindow();
  }

  async function handleDoubleClick() {
    await handleMaximizeRestore();
  }

  /** @param {Event} e */
  function stopPropagation(e) {
    e.stopPropagation();
  }
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  class="titlebar"
  data-tauri-drag-region
  ondblclick={handleDoubleClick}
>
  <!-- Left: App identity drag region -->
  <div class="titlebar__left" data-tauri-drag-region>
  </div>

  <!-- Right: Window controls -->
  <div class="titlebar__right">
    <div class="titlebar__controls">
      <button
        class="titlebar__btn titlebar__btn--minimize"
        onclick={handleMinimize}
        onmousedown={stopPropagation}
        aria-label="Minimize"
        title="Minimize"
      >
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round">
          <line x1="2" y1="6" x2="10" y2="6" />
        </svg>
      </button>

      <button
        class="titlebar__btn titlebar__btn--maximize"
        onclick={handleMaximizeRestore}
        onmousedown={stopPropagation}
        aria-label={isMaximized ? 'Restore' : 'Maximize'}
        title={isMaximized ? 'Restore' : 'Maximize'}
      >
        {#if isMaximized}
          <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="2" y="4" width="6" height="6" rx="0.5" />
            <path d="M4 4V2.5C4 2.22 4.22 2 4.5 2H9.5C9.78 2 10 2.22 10 2.5V7.5C10 7.78 9.78 8 9.5 8H8" />
          </svg>
        {:else}
          <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="2" y="2" width="8" height="8" rx="1" />
          </svg>
        {/if}
      </button>

      <button
        class="titlebar__btn titlebar__btn--close"
        onclick={handleClose}
        onmousedown={stopPropagation}
        aria-label="Close"
        title="Close"
      >
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round">
          <line x1="2.5" y1="2.5" x2="9.5" y2="9.5" />
          <line x1="9.5" y1="2.5" x2="2.5" y2="9.5" />
        </svg>
      </button>
    </div>
  </div>
</div>

<style>
  @import "./titlebar.css";
</style>

