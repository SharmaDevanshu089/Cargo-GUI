<script>
  let searchQuery = $state("");
  let filterType = $state("all"); // 'all', 'direct', 'dev', 'outdated'
  let selectedProject = $state("cargo-gui");

  // Mock dependencies
  const initialDeps = [
    { name: "tauri", type: "Direct", version: "2.0.0-rc.3", latest: "2.0.0", status: "outdated", description: "Build smaller, faster, and more secure desktop applications with a web frontend." },
    { name: "svelte", type: "Direct", version: "5.0.0-next.260", latest: "5.56.3", status: "outdated", description: "Cybernetic enhancements for HTML. A radical new approach to building user interfaces." },
    { name: "serde", type: "Direct", version: "1.0.204", latest: "1.0.204", status: "current", description: "A generic serialization/deserialization framework for Rust." },
    { name: "tokio", type: "Direct", version: "1.38.0", latest: "1.38.0", status: "current", description: "A runtime for writing reliable, asynchronous, and productive applications with the Rust programming language." },
    { name: "vite", type: "Dev", version: "5.2.11", latest: "8.0.16", status: "outdated", description: "Next generation frontend tooling. It's fast!" },
    { name: "typescript", type: "Dev", version: "5.4.5", latest: "5.4.5", status: "current", description: "TypeScript is a language for application-scale JavaScript." },
    { name: "log", type: "Build", version: "0.4.22", latest: "0.4.22", status: "current", description: "A lightweight logging facade for Rust." }
  ];

  let deps = $state(initialDeps);

  // Filter logic
  let filteredDeps = $derived(
    deps.filter(d => {
      const matchesSearch = d.name.toLowerCase().includes(searchQuery.toLowerCase()) || d.description.toLowerCase().includes(searchQuery.toLowerCase());
      
      if (!matchesSearch) return false;
      if (filterType === "all") return true;
      if (filterType === "direct") return d.type === "Direct";
      if (filterType === "dev") return d.type === "Dev";
      if (filterType === "outdated") return d.status === "outdated";
      return true;
    })
  );

  // Stats
  let outdatedCount = $derived(deps.filter(d => d.status === "outdated").length);

  function upgradeCrate(name) {
    deps = deps.map(d => {
      if (d.name === name) {
        return { ...d, version: d.latest, status: "current" };
      }
      return d;
    });
  }

  function upgradeAll() {
    deps = deps.map(d => ({ ...d, version: d.latest, status: "current" }));
  }
</script>

<div class="view-container">
  <!-- Header -->
  <header class="view-header">
    <div>
      <h2 class="view-title">Dependency Manager</h2>
      <p class="view-subtitle">Manage dependencies for the active project: <strong>{selectedProject}</strong></p>
    </div>
    
    {#if outdatedCount > 0}
      <button class="btn-primary" onclick={upgradeAll}>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.57-8.38l5.67-5.67" />
        </svg>
        Upgrade All ({outdatedCount})
      </button>
    {/if}
  </header>

  <!-- Statistics Panel -->
  <div class="stats-bar">
    <div class="stat-card">
      <span class="stat-card__title">Total Dependencies</span>
      <span class="stat-card__val">{deps.length}</span>
    </div>
    <div class="stat-card">
      <span class="stat-card__title">Up to Date</span>
      <span class="stat-card__val stat-card__val--green">{deps.filter(d => d.status === 'current').length}</span>
    </div>
    <div class="stat-card">
      <span class="stat-card__title">Outdated</span>
      <span class="stat-card__val stat-card__val--orange">{outdatedCount}</span>
    </div>
  </div>

  <!-- Toolbar & Filters -->
  <div class="toolbar">
    <div class="search-box">
      <svg class="search-box__icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="8" />
        <line x1="21" y1="21" x2="16.65" y2="16.65" />
      </svg>
      <input 
        type="text" 
        placeholder="Search dependencies..." 
        bind:value={searchQuery}
        class="search-box__input" 
      />
    </div>

    <div class="filter-group">
      <button class="filter-btn {filterType === 'all' ? 'filter-btn--active' : ''}" onclick={() => filterType = 'all'}>All</button>
      <button class="filter-btn {filterType === 'direct' ? 'filter-btn--active' : ''}" onclick={() => filterType = 'direct'}>Direct</button>
      <button class="filter-btn {filterType === 'dev' ? 'filter-btn--active' : ''}" onclick={() => filterType = 'dev'}>Dev</button>
      <button class="filter-btn {filterType === 'outdated' ? 'filter-btn--active' : ''}" onclick={() => filterType = 'outdated'}>
        Outdated 
        {#if outdatedCount > 0}
          <span class="badge-count">{outdatedCount}</span>
        {/if}
      </button>
    </div>
  </div>

  <!-- Dependencies List/Table -->
  <div class="table-container">
    <table class="deps-table">
      <thead>
        <tr>
          <th>Crate Name</th>
          <th>Type</th>
          <th>Installed</th>
          <th>Latest</th>
          <th>Status</th>
          <th class="align-right">Action</th>
        </tr>
      </thead>
      <tbody>
        {#each filteredDeps as dep}
          <tr class="deps-table__row">
            <td>
              <div class="crate-info">
                <span class="crate-name">{dep.name}</span>
                <span class="crate-desc" title={dep.description}>{dep.description}</span>
              </div>
            </td>
            <td>
              <span class="type-pill type-pill--{dep.type.toLowerCase()}">{dep.type}</span>
            </td>
            <td class="font-mono">{dep.version}</td>
            <td class="font-mono">{dep.latest}</td>
            <td>
              <span class="status-indicator status-indicator--{dep.status}">
                {dep.status === 'current' ? 'Up to date' : 'Upgrade available'}
              </span>
            </td>
            <td class="align-right">
              {#if dep.status === 'outdated'}
                <button 
                  class="btn-action" 
                  onclick={() => upgradeCrate(dep.name)}
                  title="Upgrade to {dep.latest}"
                  aria-label="Upgrade {dep.name} to {dep.latest}"
                >
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                    <polyline points="18 15 12 9 6 15" />
                  </svg>
                  Upgrade
                </button>
              {:else}
                <span class="check-icon" title="Up to date">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="20 6 9 17 4 12" />
                  </svg>
                </span>
              {/if}
            </td>
          </tr>
        {:else}
          <tr>
            <td colspan="6" class="table-empty">
              No dependencies match the filters.
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
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

  /* Stats Bar */
  .stats-bar {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 16px;
  }

  .stat-card {
    background-color: var(--cat-mantle);
    border: 1px solid var(--cat-surface0);
    border-radius: 10px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .stat-card__title {
    font-size: 12px;
    font-weight: 600;
    color: var(--cat-subtext);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .stat-card__val {
    font-size: 28px;
    font-weight: 700;
    color: var(--cat-text);
  }

  .stat-card__val--green {
    color: #a6e3a1; /* Green */
  }

  .stat-card__val--orange {
    color: #fab387; /* Peach */
  }

  /* Button Primary */
  .btn-primary {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 16px;
    background-color: #a6e3a1; /* Catppuccin Green */
    color: #11111b;
    border: none;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-primary:hover {
    background-color: #94e2d5; /* Catppuccin Teal */
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(166, 227, 161, 0.2);
  }

  /* Toolbar & Search & Filters */
  .toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 20px;
    flex-wrap: wrap;
  }

  .search-box {
    position: relative;
    flex: 1;
    min-width: 260px;
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
    padding: 10px 16px 10px 40px;
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
  }

  .filter-group {
    display: flex;
    background-color: var(--cat-mantle);
    border: 1px solid var(--cat-surface0);
    padding: 4px;
    border-radius: 8px;
    gap: 2px;
  }

  .filter-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    background: transparent;
    border: none;
    border-radius: 6px;
    color: var(--cat-subtext);
    font-family: inherit;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .filter-btn:hover {
    color: var(--cat-text);
    background-color: rgba(255, 255, 255, 0.04);
  }

  .filter-btn--active {
    background-color: var(--cat-surface0);
    color: var(--cat-text);
  }

  .badge-count {
    background-color: #fab387;
    color: #11111b;
    font-size: 10px;
    font-weight: 700;
    padding: 1px 6px;
    border-radius: 8px;
  }

  /* Table Styles */
  .table-container {
    background-color: var(--cat-mantle);
    border: 1px solid var(--cat-surface0);
    border-radius: 10px;
    overflow: hidden;
  }

  .deps-table {
    width: 100%;
    border-collapse: collapse;
    text-align: left;
    font-size: 14px;
  }

  .deps-table th {
    background-color: rgba(24, 24, 37, 0.6);
    color: var(--cat-subtext);
    font-weight: 600;
    padding: 14px 18px;
    border-bottom: 1px solid var(--cat-surface0);
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .deps-table td {
    padding: 16px 18px;
    border-bottom: 1px solid rgba(49, 50, 68, 0.4);
    color: var(--cat-text);
    vertical-align: middle;
  }

  .deps-table__row:hover {
    background-color: rgba(255, 255, 255, 0.02);
  }

  .crate-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
    max-width: 320px;
  }

  .crate-name {
    font-weight: 600;
    color: #f5e0dc; /* Rosewater */
    font-size: 15px;
  }

  .crate-desc {
    font-size: 12px;
    color: var(--cat-subtext);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .font-mono {
    font-family: 'Courier New', Courier, monospace;
    font-size: 13px;
  }

  .type-pill {
    font-size: 11px;
    font-weight: 600;
    padding: 3px 8px;
    border-radius: 6px;
    text-transform: uppercase;
    letter-spacing: 0.3px;
  }

  .type-pill--direct {
    background-color: rgba(180, 190, 254, 0.1);
    color: #b4befe;
  }

  .type-pill--dev {
    background-color: rgba(203, 166, 247, 0.1);
    color: #cba6f7;
  }

  .type-pill--build {
    background-color: rgba(249, 226, 175, 0.1);
    color: #f9e2af;
  }

  .status-indicator {
    font-size: 13px;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .status-indicator::before {
    content: "";
    width: 6px;
    height: 6px;
    border-radius: 50%;
    display: inline-block;
  }

  .status-indicator--current {
    color: #a6e3a1;
  }

  .status-indicator--current::before {
    background-color: #a6e3a1;
  }

  .status-indicator--outdated {
    color: #fab387;
  }

  .status-indicator--outdated::before {
    background-color: #fab387;
  }

  .align-right {
    text-align: right;
  }

  .btn-action {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    background-color: rgba(166, 227, 161, 0.1);
    color: #a6e3a1;
    border: 1px solid rgba(166, 227, 161, 0.2);
    border-radius: 6px;
    font-family: inherit;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-action:hover {
    background-color: #a6e3a1;
    color: #11111b;
  }

  .check-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: #a6e3a1;
    padding-right: 12px;
  }

  .table-empty {
    text-align: center;
    color: var(--cat-subtext);
    padding: 40px !important;
  }
</style>
