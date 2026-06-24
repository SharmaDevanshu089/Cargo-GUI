<script>
  let searchQuery = $state("");
  let isScanning = $state(false);
  let scanProgress = $state(0);
  let scanMessage = $state("");

  // Mock Cargo projects
  const initialProjects = [
    { name: "cargo-gui", path: "C:/Users/sharm/Documents/Cargo GUI", type: "Tauri + SvelteKit", depsCount: 19, status: "Active", lastCompiled: "2 minutes ago" },
    { name: "rocket-api", path: "C:/Users/sharm/Projects/rocket-api", type: "Rocket Web API", depsCount: 42, status: "Ready", lastCompiled: "1 hour ago" },
    { name: "bevy-game-engine", path: "C:/Users/sharm/Projects/bevy-game-engine", type: "Bevy Engine (CLI/GUI)", depsCount: 118, status: "Ready", lastCompiled: "1 day ago" },
    { name: "clap-cli-parser", path: "D:/Development/Rust/clap-cli-parser", type: "Library (CLI)", depsCount: 8, status: "Outdated", lastCompiled: "5 days ago" }
  ];

  let projects = $state(initialProjects);

  // Filtered projects based on search query
  let filteredProjects = $derived(
    projects.filter(p => p.name.toLowerCase().includes(searchQuery.toLowerCase()) || p.type.toLowerCase().includes(searchQuery.toLowerCase()))
  );

  function startScan() {
    if (isScanning) return;
    isScanning = true;
    scanProgress = 0;
    scanMessage = "Searching drive C:\\ for Cargo.toml files...";
    
    const interval = setInterval(() => {
      scanProgress += 8;
      if (scanProgress >= 30 && scanProgress < 60) {
        scanMessage = "Found 8 Cargo projects, indexing dependencies...";
      } else if (scanProgress >= 60 && scanProgress < 95) {
        scanMessage = "Resolving workspace relationships...";
      } else if (scanProgress >= 100) {
        clearInterval(interval);
        isScanning = false;
        scanProgress = 100;
        scanMessage = "Scan complete. Found 4 projects.";
        
        // Reset after a brief delay
        setTimeout(() => {
          scanProgress = 0;
          scanMessage = "";
        }, 3000);
      }
    }, 150);
  }
</script>

<div class="view-container">
  <!-- Header -->
  <header class="view-header">
    <div>
      <h2 class="view-title">Cargo Projects</h2>
      <p class="view-subtitle">Scan, index, and manage Rust crates and workspaces on your machine.</p>
    </div>
    
    <button 
      class="btn-primary {isScanning ? 'btn-primary--scanning' : ''}" 
      onclick={startScan}
      disabled={isScanning}
    >
      {#if isScanning}
        <svg class="spinner" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
          <circle cx="12" cy="12" r="10" stroke-dasharray="40" stroke-dashoffset="10" />
        </svg>
        Scanning Drive...
      {:else}
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8" />
          <line x1="21" y1="21" x2="16.65" y2="16.65" />
        </svg>
        Scan Drive
      {/if}
    </button>
  </header>

  <!-- Scan progress bar -->
  {#if isScanning || scanMessage}
    <div class="scan-banner">
      <div class="scan-banner__info">
        <span class="scan-banner__text">{scanMessage}</span>
        {#if isScanning}
          <span class="scan-banner__percentage">{scanProgress}%</span>
        {/if}
      </div>
      {#if isScanning}
        <div class="scan-banner__track">
          <div class="scan-banner__fill" style="width: {scanProgress}%"></div>
        </div>
      {/if}
    </div>
  {/if}

  <!-- Toolbar -->
  <div class="toolbar">
    <div class="search-box">
      <svg class="search-box__icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="8" />
        <line x1="21" y1="21" x2="16.65" y2="16.65" />
      </svg>
      <input 
        type="text" 
        placeholder="Search projects by name or type..." 
        bind:value={searchQuery}
        class="search-box__input" 
      />
      {#if searchQuery}
        <button class="search-box__clear" onclick={() => searchQuery = ""}>
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
        </button>
      {/if}
    </div>
  </div>

  <!-- Projects Grid -->
  <div class="projects-grid">
    {#each filteredProjects as project}
      <div class="project-card">
        <div class="project-card__header">
          <div class="project-card__title-group">
            <h3 class="project-card__name">{project.name}</h3>
            <span class="project-card__badge project-card__badge--{project.status.toLowerCase()}">{project.status}</span>
          </div>
          <span class="project-card__type">{project.type}</span>
        </div>
        
        <div class="project-card__body">
          <div class="project-card__path" title={project.path}>
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" />
            </svg>
            <span>{project.path}</span>
          </div>
        </div>

        <div class="project-card__footer">
          <div class="project-card__meta">
            <span class="project-card__meta-item">
              <strong>{project.depsCount}</strong> dependencies
            </span>
            <span class="project-card__meta-dot">•</span>
            <span class="project-card__meta-item">Compiled {project.lastCompiled}</span>
          </div>
          <button class="project-card__btn" aria-label="Open project {project.name}">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="9 18 15 12 9 6" />
            </svg>
          </button>
        </div>
      </div>
    {:else}
      <div class="empty-state">
        <svg class="empty-state__icon" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8" />
          <line x1="21" y1="21" x2="16.65" y2="16.65" />
        </svg>
        <h3 class="empty-state__title">No projects found</h3>
        <p class="empty-state__description">Try searching for something else or scan your drive to find Cargo projects.</p>
      </div>
    {/each}
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
    gap: 16px;
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

  /* Primary Button */
  .btn-primary {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 16px;
    background-color: #b4befe; /* Lavender */
    color: #11111b; /* Crust */
    border: none;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-primary:hover:not(:disabled) {
    background-color: #cba6f7; /* Mauve hover */
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(180, 190, 254, 0.2);
  }

  .btn-primary:active:not(:disabled) {
    transform: translateY(0);
  }

  .btn-primary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-primary--scanning {
    background-color: var(--cat-surface0);
    color: var(--cat-subtext);
  }

  .spinner {
    animation: rotate 1.5s linear infinite;
  }

  @keyframes rotate {
    100% { transform: rotate(360deg); }
  }

  /* Scan Banner */
  .scan-banner {
    background-color: var(--cat-mantle);
    border: 1px solid var(--cat-surface0);
    border-radius: 8px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .scan-banner__info {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 13px;
  }

  .scan-banner__text {
    color: var(--cat-text);
  }

  .scan-banner__percentage {
    color: #b4befe;
    font-weight: 600;
  }

  .scan-banner__track {
    height: 6px;
    background-color: var(--cat-surface0);
    border-radius: 3px;
    overflow: hidden;
  }

  .scan-banner__fill {
    height: 100%;
    background-color: #b4befe;
    border-radius: 3px;
    transition: width 0.15s ease;
  }

  /* Toolbar */
  .toolbar {
    display: flex;
    gap: 16px;
  }

  .search-box {
    position: relative;
    flex: 1;
    display: flex;
    align-items: center;
  }

  .search-box__icon {
    position: absolute;
    left: 14px;
    color: var(--cat-subtext);
    pointer-events: none;
  }

  .search-box__input {
    width: 100%;
    padding: 11px 16px 11px 40px;
    background-color: var(--cat-mantle);
    border: 1px solid var(--cat-surface0);
    border-radius: 8px;
    color: var(--cat-text);
    font-family: inherit;
    font-size: 14px;
    outline: none;
    transition: all 0.2s ease;
  }

  .search-box__input:focus {
    border-color: #b4befe;
    box-shadow: 0 0 0 2px rgba(180, 190, 254, 0.15);
  }

  .search-box__clear {
    position: absolute;
    right: 12px;
    background: none;
    border: none;
    color: var(--cat-subtext);
    cursor: pointer;
    display: flex;
    align-items: center;
    padding: 4px;
    border-radius: 4px;
  }

  .search-box__clear:hover {
    background-color: var(--cat-surface0);
    color: var(--cat-text);
  }

  /* Grid & Cards */
  .projects-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 20px;
  }

  .project-card {
    background-color: var(--cat-mantle);
    border: 1px solid var(--cat-surface0);
    border-radius: 12px;
    padding: 20px;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    gap: 16px;
    transition: all 0.2s ease;
  }

  .project-card:hover {
    border-color: rgba(180, 190, 254, 0.5);
    box-shadow: 0 6px 16px rgba(0, 0, 0, 0.2);
    transform: translateY(-2px);
  }

  .project-card__header {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .project-card__title-group {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 12px;
  }

  .project-card__name {
    font-size: 16px;
    font-weight: 600;
    color: var(--cat-text);
    margin: 0;
  }

  .project-card__type {
    font-size: 12px;
    color: var(--cat-subtext);
  }

  .project-card__badge {
    font-size: 11px;
    font-weight: 600;
    padding: 2px 8px;
    border-radius: 10px;
  }

  .project-card__badge--active {
    background-color: rgba(166, 227, 161, 0.1); /* Catppuccin Green */
    color: #a6e3a1;
  }

  .project-card__badge--ready {
    background-color: rgba(137, 180, 250, 0.1); /* Catppuccin Blue */
    color: #89b4fa;
  }

  .project-card__badge--outdated {
    background-color: rgba(250, 179, 135, 0.1); /* Catppuccin Peach */
    color: #fab387;
  }

  .project-card__path {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    color: var(--cat-subtext);
    background-color: var(--cat-base);
    padding: 8px 12px;
    border-radius: 6px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .project-card__path svg {
    flex-shrink: 0;
  }

  .project-card__footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: 12px;
    border-top: 1px solid rgba(49, 50, 68, 0.3);
  }

  .project-card__meta {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: var(--cat-subtext);
  }

  .project-card__meta-dot {
    color: var(--cat-surface0);
  }

  .project-card__btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    background-color: var(--cat-surface0);
    border: none;
    border-radius: 6px;
    color: var(--cat-text);
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .project-card__btn:hover {
    background-color: #b4befe;
    color: #11111b;
  }

  /* Empty State */
  .empty-state {
    grid-column: 1 / -1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 60px 20px;
    text-align: center;
    background-color: var(--cat-mantle);
    border: 1px dashed var(--cat-surface0);
    border-radius: 12px;
  }

  .empty-state__icon {
    color: var(--cat-surface0);
    margin-bottom: 16px;
  }

  .empty-state__title {
    font-size: 16px;
    font-weight: 600;
    color: var(--cat-text);
    margin: 0;
  }

  .empty-state__description {
    font-size: 14px;
    color: var(--cat-subtext);
    max-width: 360px;
    margin: 8px 0 0 0;
  }
</style>
