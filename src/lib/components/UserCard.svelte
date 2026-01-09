<script lang="ts">
  import { Settings, LogOut, ChevronDown, ChevronUp } from 'lucide-svelte';

  interface Props {
    username: string;
    subscription: string;
    avatarUrl?: string;
    onSettingsClick: () => void;
    onLogout?: () => void;
  }

  let { username, subscription, avatarUrl, onSettingsClick, onLogout }: Props = $props();

  let isCollapsed = $state(false);
</script>

{#if isCollapsed}
  <!-- Collapsed view -->
  <button class="collapsed-card" onclick={() => isCollapsed = false}>
    <div class="mini-logo">Q</div>
    <span class="collapsed-username">{username}</span>
    <ChevronDown size={12} />
  </button>
{:else}
  <!-- Expanded view -->
  <div class="user-card">
    <!-- Avatar -->
    <div class="avatar">
      {#if avatarUrl}
        <img src={avatarUrl} alt={username} />
      {:else}
        {username.charAt(0).toUpperCase()}
      {/if}
    </div>

    <!-- User Info -->
    <div class="user-info">
      <div class="username">{username}</div>
      <div class="subscription">{subscription}</div>
    </div>

    <!-- Action Buttons -->
    <div class="action-buttons">
      <button
        class="action-btn"
        onclick={(e) => {
          e.stopPropagation();
          onSettingsClick();
        }}
        title="Settings"
      >
        <Settings size={14} />
      </button>
      {#if onLogout}
        <button
          class="action-btn logout-btn"
          onclick={(e) => {
            e.stopPropagation();
            onLogout();
          }}
          title="Logout"
        >
          <LogOut size={14} />
        </button>
      {/if}
      <button
        class="action-btn collapse-btn"
        onclick={(e) => {
          e.stopPropagation();
          isCollapsed = true;
        }}
        title="Collapse"
      >
        <ChevronUp size={14} />
      </button>
    </div>
  </div>
{/if}

<style>
  .user-card {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px;
    border-radius: 6px;
    cursor: pointer;
    transition: background-color 150ms ease;
  }

  .user-card:hover {
    background-color: var(--bg-hover);
  }

  .avatar {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    background-color: var(--accent-primary);
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    font-weight: 600;
    font-size: 11px;
    flex-shrink: 0;
    overflow: hidden;
  }

  .avatar img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .user-info {
    flex: 1;
    min-width: 0;
  }

  .username {
    font-size: 12px;
    font-weight: 500;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .subscription {
    font-size: 10px;
    color: var(--accent-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .action-buttons {
    display: flex;
    align-items: center;
    gap: 2px;
  }

  .action-btn {
    width: 22px;
    height: 22px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    flex-shrink: 0;
    border-radius: 4px;
    transition: all 150ms ease;
  }

  .action-btn:hover {
    color: var(--text-primary);
    background-color: var(--bg-tertiary);
  }

  .logout-btn:hover {
    color: #ff6b6b;
  }

  /* Collapsed state */
  .collapsed-card {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 6px 8px;
    border-radius: 6px;
    background: none;
    border: none;
    cursor: pointer;
    transition: background-color 150ms ease;
    color: var(--text-primary);
  }

  .collapsed-card:hover {
    background-color: var(--bg-hover);
  }

  .mini-logo {
    width: 20px;
    height: 20px;
    border-radius: 4px;
    background-color: var(--accent-primary);
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    font-weight: 700;
    font-size: 10px;
    flex-shrink: 0;
  }

  .collapsed-username {
    flex: 1;
    font-size: 12px;
    font-weight: 500;
    text-align: left;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .collapsed-card :global(svg) {
    color: var(--text-muted);
    flex-shrink: 0;
  }
</style>
