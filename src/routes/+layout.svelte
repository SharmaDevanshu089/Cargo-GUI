<script>
  let { children } = $props();
</script>

<div class="app-container">
  {@render children()}
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
    height: 100vh;
    width: 100vw;
    overflow: hidden;
  }
</style>
