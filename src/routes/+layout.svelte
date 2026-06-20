<script>
  import Titlebar from './components/titlebar/Titlebar.svelte';
  
  let { children } = $props();
</script>

<div class="app-container">
  <Titlebar />
  <main class="content-container">
    {@render children()}
  </main>
</div>

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    font-family: 'Inter', sans-serif;
    background-color: var(--cat-base, #1e1e2e);
    color: var(--cat-text, #cdd6f4);
    height: 100vh;
    width: 100vw;
    overflow: hidden;
  }

  .app-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
  }

  .content-container {
    flex: 1;
    overflow: auto;
  }
</style>
