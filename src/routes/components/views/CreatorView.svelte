<script>
  let projName = $state("");
  let parentDir = $state("C:/Users/sharm/Documents");
  let projType = $state("bin"); // 'bin' or 'lib'
  let useGit = $state(true);
  let isCreating = $state(false);
  let createLogs = $state([]);
  let showLogs = $state(false);
  let isCreated = $state(false);

  function handleCreate() {
    if (!projName) return;
    isCreating = true;
    showLogs = true;
    isCreated = false;
    createLogs = [];

    const logsList = [
      `$ cargo init "${parentDir}/${projName}" --${projType} ${useGit ? '--vcs git' : '--vcs none'}`,
      "Creating directory structure...",
      `Creating Cargo.toml package specification...`,
      projType === 'bin' ? "Creating src/main.rs (binary entrypoint)..." : "Creating src/lib.rs (library entrypoint)...",
      useGit ? "Initializing git repository..." : "Skipping git repository initialization...",
      `Successfully created ${projType === 'bin' ? 'binary (application)' : 'library'} \`${projName}\` package.`
    ];

    let index = 0;
    const interval = setInterval(() => {
      if (index < logsList.length) {
        createLogs = [...createLogs, logsList[index]];
        index++;
      } else {
        clearInterval(interval);
        isCreating = false;
        isCreated = true;
      }
    }, 400);
  }

  function resetForm() {
    projName = "";
    showLogs = false;
    isCreated = false;
    createLogs = [];
  }
</script>

<div class="view-container">
  <!-- Header -->
  <header class="view-header">
    <div>
      <h2 class="view-title">Create Cargo Project</h2>
      <p class="view-subtitle">Initialize a new Cargo package with customized options.</p>
    </div>
  </header>

  <div class="creator-layout">
    <!-- Form Area -->
    <div class="creator-card">
      <form onsubmit={(e) => { e.preventDefault(); handleCreate(); }} class="creator-form">
        <!-- Project Name -->
        <div class="form-group">
          <label for="proj-name" class="form-label">Project Name</label>
          <input 
            type="text" 
            id="proj-name"
            placeholder="my-awesome-crate" 
            bind:value={projName}
            required
            disabled={isCreating}
            class="form-input"
            pattern="[a-zA-Z0-9_-]+"
            title="Only alphanumeric characters, dashes, and underscores are allowed"
          />
          <span class="form-help">Must be a valid Cargo package name (lowercase, no spaces, use dashes/underscores).</span>
        </div>

        <!-- Directory Location -->
        <div class="form-group">
          <label for="proj-dir" class="form-label">Location</label>
          <div class="browse-input">
            <input 
              type="text" 
              id="proj-dir"
              bind:value={parentDir}
              required
              disabled={isCreating}
              class="form-input" 
            />
            <button type="button" class="btn-secondary" disabled={isCreating}>Browse</button>
          </div>
        </div>

        <!-- Project Type Selector Cards -->
        <div class="form-group">
          <label class="form-label">Project Type</label>
          <div class="type-grid">
            <!-- Binary -->
            <button 
              type="button"
              class="type-card {projType === 'bin' ? 'type-card--active' : ''}" 
              onclick={() => projType = 'bin'}
              disabled={isCreating}
            >
              <div class="type-card__header">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="4 17 10 11 4 5" />
                  <line x1="12" y1="19" x2="20" y2="19" />
                </svg>
                <span class="type-card__title">Binary (--bin)</span>
              </div>
              <p class="type-card__desc">An executable application package with a main function (src/main.rs).</p>
            </button>

            <!-- Library -->
            <button 
              type="button"
              class="type-card {projType === 'lib' ? 'type-card--active' : ''}" 
              onclick={() => projType = 'lib'}
              disabled={isCreating}
            >
              <div class="type-card__header">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20" />
                  <path d="M4 4.5A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1-2.5-2.5v-15z" />
                </svg>
                <span class="type-card__title">Library (--lib)</span>
              </div>
              <p class="type-card__desc">A reusable library package to share code with other crates (src/lib.rs).</p>
            </button>
          </div>
        </div>

        <!-- VCS Toggle -->
        <div class="toggle-group">
          <div class="toggle-info">
            <span class="toggle-title">Initialize Git Repository</span>
            <span class="toggle-desc">Automatically run 'git init' and create a default .gitignore file.</span>
          </div>
          <label class="switch" aria-label="Initialize Git Repository Toggle">
            <input type="checkbox" bind:checked={useGit} disabled={isCreating} />
            <span class="slider"></span>
          </label>
        </div>

        <!-- Submit Button -->
        <div class="form-actions">
          {#if isCreated}
            <button type="button" class="btn-secondary" onclick={resetForm}>Create Another</button>
          {/if}
          <button 
            type="submit" 
            class="btn-primary {isCreating ? 'btn-primary--loading' : ''}"
            disabled={isCreating || !projName}
          >
            {#if isCreating}
              <svg class="spinner" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                <circle cx="12" cy="12" r="10" stroke-dasharray="40" stroke-dashoffset="10" />
              </svg>
              Creating...
            {:else}
              Initialize Project
            {/if}
          </button>
        </div>
      </form>
    </div>

    <!-- Live Terminal Output -->
    {#if showLogs}
      <div class="terminal-card">
        <div class="terminal-header">
          <div class="terminal-dots">
            <span class="dot red"></span>
            <span class="dot yellow"></span>
            <span class="dot green"></span>
          </div>
          <span class="terminal-title">cargo-execution.log</span>
        </div>
        <div class="terminal-body font-mono">
          {#each createLogs as log, i}
            <div class="terminal-line {i === 0 ? 'terminal-line--cmd' : ''} {log.startsWith('Successfully') ? 'terminal-line--success' : ''}">
              {log}
            </div>
          {/each}
          {#if isCreating}
            <div class="terminal-line terminal-line--pulse">_</div>
          {/if}
        </div>
      </div>
    {/if}
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

  /* Layout */
  .creator-layout {
    display: grid;
    grid-template-columns: 1fr;
    gap: 24px;
  }

  @media (min-width: 900px) {
    .creator-layout {
      grid-template-columns: 1.2fr 0.8fr;
    }
  }

  .creator-card {
    background-color: var(--cat-mantle);
    border: 1px solid var(--cat-surface0);
    border-radius: 12px;
    padding: 24px;
  }

  /* Form Styles */
  .creator-form {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .form-label {
    font-size: 14px;
    font-weight: 600;
    color: var(--cat-text);
  }

  .form-input {
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

  .form-input:focus {
    border-color: #b4befe;
  }

  .form-input:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .form-help {
    font-size: 11px;
    color: var(--cat-subtext);
  }

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

  .btn-secondary:hover:not(:disabled) {
    background-color: #45475a;
  }

  .btn-secondary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  /* Type Selector Cards */
  .type-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 16px;
  }

  @media (min-width: 600px) {
    .type-grid {
      grid-template-columns: 1fr 1fr;
    }
  }

  .type-card {
    background-color: var(--cat-base);
    border: 1px solid var(--cat-surface0);
    border-radius: 10px;
    padding: 18px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    cursor: pointer;
    text-align: left;
    outline: none;
    transition: all 0.2s ease;
  }

  .type-card:hover:not(:disabled) {
    border-color: rgba(180, 190, 254, 0.4);
    transform: translateY(-1px);
  }

  .type-card--active {
    border-color: #b4befe;
    background-color: rgba(180, 190, 254, 0.04);
  }

  .type-card__header {
    display: flex;
    align-items: center;
    gap: 10px;
    color: var(--cat-text);
  }

  .type-card--active .type-card__header {
    color: #b4befe;
  }

  .type-card__title {
    font-size: 15px;
    font-weight: 600;
  }

  .type-card__desc {
    font-size: 12px;
    color: var(--cat-subtext);
    margin: 0;
    line-height: 1.5;
  }

  /* VCS Toggle */
  .toggle-group {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background-color: var(--cat-base);
    border: 1px solid var(--cat-surface0);
    padding: 14px 18px;
    border-radius: 10px;
  }

  .toggle-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .toggle-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--cat-text);
  }

  .toggle-desc {
    font-size: 12px;
    color: var(--cat-subtext);
  }

  /* Switch Slider */
  .switch {
    position: relative;
    display: inline-block;
    width: 44px;
    height: 24px;
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

  input:disabled + .slider {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Button Primary */
  .btn-primary {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 18px;
    background-color: #b4befe;
    color: #11111b;
    border: none;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-primary:hover:not(:disabled) {
    background-color: #cba6f7;
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(180, 190, 254, 0.25);
  }

  .btn-primary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .spinner {
    animation: rotate 1.5s linear infinite;
  }

  @keyframes rotate {
    100% { transform: rotate(360deg); }
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    margin-top: 10px;
  }

  /* Terminal Card */
  .terminal-card {
    background-color: #11111b; /* Crust */
    border: 1px solid var(--cat-surface0);
    border-radius: 12px;
    overflow: hidden;
    height: fit-content;
    max-height: 380px;
    display: flex;
    flex-direction: column;
  }

  .terminal-header {
    background-color: var(--cat-mantle);
    padding: 12px 16px;
    display: flex;
    align-items: center;
    position: relative;
    border-bottom: 1px solid var(--cat-surface0);
  }

  .terminal-dots {
    display: flex;
    gap: 6px;
  }

  .dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
  }

  .dot.red { background-color: #f38ba8; }
  .dot.yellow { background-color: #f9e2af; }
  .dot.green { background-color: #a6e3a1; }

  .terminal-title {
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
    font-size: 12px;
    color: var(--cat-subtext);
    font-weight: 500;
  }

  .terminal-body {
    padding: 16px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 8px;
    font-size: 13px;
    color: #a6adc8; /* Subtext */
    min-height: 240px;
  }

  .terminal-line {
    line-height: 1.5;
    word-break: break-all;
  }

  .terminal-line--cmd {
    color: #cba6f7; /* Mauve command line */
  }

  .terminal-line--success {
    color: #a6e3a1; /* Green success message */
    font-weight: 600;
  }

  .terminal-line--pulse {
    animation: blink 1s step-end infinite;
    color: var(--cat-text);
  }

  @keyframes blink {
    from, to { color: transparent }
    50% { color: var(--cat-text) }
  }
</style>
