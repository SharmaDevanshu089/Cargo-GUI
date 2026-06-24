<script>
  import Titlebar from './components/titlebar/Titlebar.svelte';
  import Sidebar from './components/sidebar/Sidebar.svelte';
  import { currentWindow } from './components/sidebar/navigation.js';

  import ProjectsView from './components/views/ProjectsView.svelte';
  import DependenciesView from './components/views/DependenciesView.svelte';
  import CreatorView from './components/views/CreatorView.svelte';
  import SettingsView from './components/views/SettingsView.svelte';
</script>

<div class="page-shell">
  <!-- Sidebar on the left, spanning the full height -->
  <Sidebar />

  <!-- Main layout on the right: Titlebar on top, workspace below -->
  <div class="main-layout">
    <Titlebar />

    <main class="workspace__content">
      {#if $currentWindow === 'projects'}
        <div class="fade-in">
          <ProjectsView />
        </div>
      {:else if $currentWindow === 'dependencies'}
        <div class="fade-in">
          <DependenciesView />
        </div>
      {:else if $currentWindow === 'creator'}
        <div class="fade-in">
          <CreatorView />
        </div>
      {:else if $currentWindow === 'settings'}
        <div class="fade-in">
          <SettingsView />
        </div>
      {/if}
    </main>
  </div>
</div>

<style>
  .page-shell {
    display: flex;
    flex-direction: row; /* Sidebar on left, content on right */
    height: 100vh;
    width: 100vw;
    overflow: hidden;
    background-color: var(--cat-base, #1e1e2e);
  }

  .main-layout {
    display: flex;
    flex-direction: column; /* Titlebar on top, active view on bottom */
    flex: 1;
    height: 100%;
    overflow: hidden;
  }

  .workspace__content {
    flex: 1;
    overflow-y: auto;
    padding: 28px 32px;
    background-color: var(--cat-base, #1e1e2e);
    box-sizing: border-box;
  }

  /* Micro-animation for switching tabs */
  .fade-in {
    animation: fadeIn 0.25s ease-out;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(4px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
