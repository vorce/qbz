<script lang="ts">
  import { Settings } from 'lucide-svelte';

  interface Props {
    username: string;
    subscription: string;
    avatarUrl?: string;
    onSettingsClick: () => void;
    onLogout?: () => void;
  }

  let { username, subscription, avatarUrl, onSettingsClick }: Props = $props();
</script>

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

  <!-- Settings Button -->
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
</div>

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
</style>
