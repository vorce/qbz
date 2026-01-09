<script lang="ts">
  import { Search, Home, Radio, Clock, Music, Disc3, Mic2, Plus, MoreVertical, ChevronDown } from 'lucide-svelte';
  import NavigationItem from './NavigationItem.svelte';
  import UserCard from './UserCard.svelte';

  interface Props {
    activeView: string;
    onNavigate: (view: string) => void;
    onSettingsClick?: () => void;
  }

  let { activeView, onNavigate, onSettingsClick }: Props = $props();

  function handleViewChange(view: string) {
    onNavigate(view);
  }
</script>

<aside class="sidebar">
  <!-- Scrollable Content Area -->
  <div class="content">
    <!-- Search Bar -->
    <div class="search-container">
      <Search class="search-icon" size={16} />
      <input
        type="text"
        placeholder="Search"
        class="search-input"
      />
    </div>

    <!-- Navigation Section -->
    <nav class="nav-section">
      <NavigationItem
        label="Home"
        active={activeView === 'home'}
        onclick={() => handleViewChange('home')}
      >
        {#snippet icon()}<Home size={18} />{/snippet}
      </NavigationItem>
      <NavigationItem
        label="New"
        active={activeView === 'new'}
        onclick={() => handleViewChange('new')}
      >
        {#snippet icon()}<Radio size={18} />{/snippet}
      </NavigationItem>
      <NavigationItem
        label="Radio"
        active={activeView === 'radio'}
        onclick={() => handleViewChange('radio')}
      >
        {#snippet icon()}<Radio size={18} />{/snippet}
      </NavigationItem>
    </nav>

    <!-- Library Section -->
    <div class="section">
      <div class="section-header">Library</div>
      <NavigationItem
        label="Recently Added"
        active={activeView === 'recently-added'}
        onclick={() => handleViewChange('recently-added')}
      >
        {#snippet icon()}<Clock size={18} />{/snippet}
      </NavigationItem>
      <NavigationItem
        label="Songs"
        active={activeView === 'songs'}
        onclick={() => handleViewChange('songs')}
      >
        {#snippet icon()}<Music size={18} />{/snippet}
      </NavigationItem>
      <NavigationItem
        label="Albums"
        active={activeView === 'albums'}
        onclick={() => handleViewChange('albums')}
      >
        {#snippet icon()}<Disc3 size={18} />{/snippet}
      </NavigationItem>
      <NavigationItem
        label="Artists"
        active={activeView === 'artists'}
        onclick={() => handleViewChange('artists')}
      >
        {#snippet icon()}<Mic2 size={18} />{/snippet}
      </NavigationItem>
    </div>

    <!-- Playlists Section -->
    <div class="section playlists-section">
      <div class="playlists-header">
        <div class="section-header">Playlists</div>
        <div class="header-actions">
          <button class="icon-btn">
            <MoreVertical size={14} />
          </button>
          <button class="icon-btn">
            <ChevronDown size={14} />
          </button>
        </div>
      </div>

      <button class="create-btn">
        <Plus size={18} />
        <span>Create New...</span>
      </button>

      <NavigationItem
        label="All Playlists"
        active={activeView === 'all-playlists'}
        onclick={() => handleViewChange('all-playlists')}
      >
        {#snippet icon()}<Music size={18} />{/snippet}
      </NavigationItem>
      <NavigationItem
        label="My Playlist 1"
        active={activeView === 'playlist-1'}
        onclick={() => handleViewChange('playlist-1')}
      >
        {#snippet icon()}<Music size={18} />{/snippet}
      </NavigationItem>
      <NavigationItem
        label="My Playlist 2"
        active={activeView === 'playlist-2'}
        onclick={() => handleViewChange('playlist-2')}
      >
        {#snippet icon()}<Music size={18} />{/snippet}
      </NavigationItem>
      <NavigationItem
        label="My Playlist 3"
        active={activeView === 'playlist-3'}
        onclick={() => handleViewChange('playlist-3')}
      >
        {#snippet icon()}<Music size={18} />{/snippet}
      </NavigationItem>
    </div>
  </div>

  <!-- Fixed User Profile at Bottom -->
  <div class="user-section">
    <UserCard
      username="Username"
      subscription="Studio Premier"
      onSettingsClick={onSettingsClick ?? (() => handleViewChange('settings'))}
    />
  </div>
</aside>

<style>
  .sidebar {
    width: 240px;
    background-color: var(--bg-secondary);
    display: flex;
    flex-direction: column;
    height: calc(100vh - 80px);
  }

  .content {
    flex: 1;
    overflow-y: auto;
    padding: 16px;
    padding-bottom: 0;
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .search-container {
    position: relative;
  }

  .search-container :global(.search-icon) {
    position: absolute;
    left: 12px;
    top: 50%;
    transform: translateY(-50%);
    color: var(--text-muted);
  }

  .search-input {
    width: 100%;
    height: 36px;
    background-color: var(--bg-tertiary);
    border-radius: 8px;
    padding-left: 40px;
    padding-right: 12px;
    font-size: 14px;
    color: var(--text-primary);
    border: none;
    outline: none;
  }

  .search-input::placeholder {
    color: var(--text-muted);
  }

  .nav-section {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .section {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .section-header {
    font-size: 11px;
    text-transform: uppercase;
    color: var(--text-muted);
    font-weight: 600;
    letter-spacing: 0.05em;
    margin-bottom: 8px;
    padding: 0 12px;
  }

  .playlists-section {
    flex: 1;
    padding-bottom: 16px;
  }

  .playlists-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 12px;
    margin-bottom: 8px;
  }

  .playlists-header .section-header {
    margin-bottom: 0;
    padding: 0;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .icon-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 2px;
    transition: color 150ms ease;
  }

  .icon-btn:hover {
    color: var(--text-primary);
  }

  .create-btn {
    width: 100%;
    height: 36px;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 12px;
    border-radius: 8px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    transition: background-color 150ms ease;
  }

  .create-btn:hover {
    background-color: var(--bg-hover);
  }

  .create-btn span {
    font-size: 14px;
    font-weight: 400;
  }

  .user-section {
    border-top: 1px solid var(--bg-tertiary);
    padding: 12px;
  }
</style>
