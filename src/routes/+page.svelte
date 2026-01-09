<script lang="ts">
  import { onMount } from 'svelte';

  // Components
  import Sidebar from '$lib/components/Sidebar.svelte';
  import NowPlayingBar from '$lib/components/NowPlayingBar.svelte';
  import Toast from '$lib/components/Toast.svelte';

  // Views
  import HomeView from '$lib/components/views/HomeView.svelte';
  import SettingsView from '$lib/components/views/SettingsView.svelte';
  import AlbumDetailView from '$lib/components/views/AlbumDetailView.svelte';

  // Overlays
  import QueuePanel from '$lib/components/QueuePanel.svelte';
  import FullScreenNowPlaying from '$lib/components/FullScreenNowPlaying.svelte';
  import FocusMode from '$lib/components/FocusMode.svelte';

  // Types
  interface Album {
    id: string;
    artwork: string;
    title: string;
    artist: string;
  }

  interface Track {
    number: number;
    title: string;
    artist?: string;
    duration: string;
    quality?: string;
  }

  interface AlbumDetail {
    artwork: string;
    title: string;
    artist: string;
    year: string;
    label: string;
    genre: string;
    quality: string;
    trackCount: number;
    duration: string;
    tracks: Track[];
  }

  interface QueueTrack {
    id: string;
    artwork: string;
    title: string;
    artist: string;
    duration: string;
  }

  // View State
  let activeView = $state<'home' | 'search' | 'library' | 'settings' | 'album'>('home');
  let viewHistory = $state<string[]>(['home']);
  let selectedAlbum = $state<AlbumDetail | null>(null);

  // Overlay States
  let isQueueOpen = $state(false);
  let isFullScreenOpen = $state(false);
  let isFocusModeOpen = $state(false);

  // Playback State
  let isPlaying = $state(false);
  let currentTime = $state(73);
  let duration = $state(245);
  let volume = $state(75);
  let isShuffle = $state(false);
  let repeatMode = $state<'off' | 'all' | 'one'>('off');
  let isFavorite = $state(false);

  // Toast State
  let toast = $state<{ message: string; type: 'success' | 'error' | 'info' } | null>(null);

  // Mock Data
  const mockAlbums: Album[] = [
    { id: '1', artwork: 'https://picsum.photos/seed/album1/300/300', title: 'Midnight Dreams', artist: 'Luna Nova' },
    { id: '2', artwork: 'https://picsum.photos/seed/album2/300/300', title: 'Electric Soul', artist: 'The Voltage' },
    { id: '3', artwork: 'https://picsum.photos/seed/album3/300/300', title: 'Cosmic Journey', artist: 'Stellar Wind' },
    { id: '4', artwork: 'https://picsum.photos/seed/album4/300/300', title: 'Urban Echoes', artist: 'City Lights' },
    { id: '5', artwork: 'https://picsum.photos/seed/album5/300/300', title: 'Ocean Waves', artist: 'Deep Blue' },
    { id: '6', artwork: 'https://picsum.photos/seed/album6/300/300', title: 'Mountain High', artist: 'Peak Sound' },
    { id: '7', artwork: 'https://picsum.photos/seed/album7/300/300', title: 'Desert Storm', artist: 'Sand Dunes' },
    { id: '8', artwork: 'https://picsum.photos/seed/album8/300/300', title: 'Forest Tales', artist: 'Green Echo' },
  ];

  const mockAlbumDetail: AlbumDetail = {
    artwork: 'https://picsum.photos/seed/album1/500/500',
    title: 'Midnight Dreams',
    artist: 'Luna Nova',
    year: '2024',
    label: 'Stellar Records',
    genre: 'Electronic',
    quality: '24-Bit / 96 kHz',
    trackCount: 12,
    duration: '48:32',
    tracks: [
      { number: 1, title: 'Moonrise', duration: '4:23', quality: 'Hi-Res' },
      { number: 2, title: 'Starlight Serenade', duration: '3:45', quality: 'Hi-Res' },
      { number: 3, title: 'Night Sky', duration: '5:12', quality: 'Hi-Res' },
      { number: 4, title: 'Celestial Dance', duration: '4:01', quality: 'Hi-Res' },
      { number: 5, title: 'Lunar Eclipse', duration: '3:56', quality: 'Hi-Res' },
      { number: 6, title: 'Cosmic Dust', duration: '4:33', quality: 'Hi-Res' },
      { number: 7, title: 'Nebula Dreams', duration: '5:08', quality: 'Hi-Res' },
      { number: 8, title: 'Aurora', duration: '3:42', quality: 'Hi-Res' },
      { number: 9, title: 'Twilight Zone', duration: '4:15', quality: 'Hi-Res' },
      { number: 10, title: 'Midnight Hour', duration: '3:58', quality: 'Hi-Res' },
      { number: 11, title: 'Dream Sequence', duration: '4:47', quality: 'Hi-Res' },
      { number: 12, title: 'Dawn Breaking', duration: '4:12', quality: 'Hi-Res' },
    ],
  };

  const currentTrack: QueueTrack = {
    id: 'current',
    artwork: 'https://picsum.photos/seed/album1/300/300',
    title: 'Moonrise',
    artist: 'Luna Nova',
    duration: '4:23',
  };

  const upcomingTracks: QueueTrack[] = [
    { id: '2', artwork: 'https://picsum.photos/seed/album1/300/300', title: 'Starlight Serenade', artist: 'Luna Nova', duration: '3:45' },
    { id: '3', artwork: 'https://picsum.photos/seed/album1/300/300', title: 'Night Sky', artist: 'Luna Nova', duration: '5:12' },
    { id: '4', artwork: 'https://picsum.photos/seed/album1/300/300', title: 'Celestial Dance', artist: 'Luna Nova', duration: '4:01' },
  ];

  // View Types
  type ViewType = 'home' | 'search' | 'library' | 'settings' | 'album';

  // Navigation Functions
  function navigateTo(view: string) {
    const typedView = view as ViewType;
    if (typedView !== activeView) {
      viewHistory = [...viewHistory, typedView];
      activeView = typedView;
    }
  }

  function goBack() {
    if (viewHistory.length > 1) {
      viewHistory = viewHistory.slice(0, -1);
      activeView = viewHistory[viewHistory.length - 1] as typeof activeView;
      if (activeView !== 'album') {
        selectedAlbum = null;
      }
    }
  }

  function handleAlbumClick(albumId: string) {
    selectedAlbum = mockAlbumDetail;
    navigateTo('album');
  }

  // Playback Functions
  function togglePlay() {
    isPlaying = !isPlaying;
  }

  function handleSeek(time: number) {
    currentTime = Math.max(0, Math.min(duration, time));
  }

  function handleVolumeChange(newVolume: number) {
    volume = Math.max(0, Math.min(100, newVolume));
  }

  function toggleShuffle() {
    isShuffle = !isShuffle;
    showToast(isShuffle ? 'Shuffle enabled' : 'Shuffle disabled', 'info');
  }

  function toggleRepeat() {
    if (repeatMode === 'off') repeatMode = 'all';
    else if (repeatMode === 'all') repeatMode = 'one';
    else repeatMode = 'off';

    const messages = { off: 'Repeat off', all: 'Repeat all', one: 'Repeat one' };
    showToast(messages[repeatMode], 'info');
  }

  function toggleFavorite() {
    isFavorite = !isFavorite;
    showToast(isFavorite ? 'Added to favorites' : 'Removed from favorites', 'success');
  }

  // Toast Function
  function showToast(message: string, type: 'success' | 'error' | 'info' = 'info') {
    toast = { message, type };
  }

  function hideToast() {
    toast = null;
  }

  // Keyboard Shortcuts
  function handleKeydown(e: KeyboardEvent) {
    // Don't trigger shortcuts when typing in inputs
    if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
      return;
    }

    switch (e.key) {
      case ' ':
        e.preventDefault();
        togglePlay();
        break;
      case 'f':
        if (!e.ctrlKey && !e.metaKey) {
          isFocusModeOpen = !isFocusModeOpen;
        }
        break;
      case 'q':
        isQueueOpen = !isQueueOpen;
        break;
      case 'Escape':
        if (isFocusModeOpen) isFocusModeOpen = false;
        else if (isFullScreenOpen) isFullScreenOpen = false;
        else if (isQueueOpen) isQueueOpen = false;
        break;
    }
  }

  onMount(() => {
    document.addEventListener('keydown', handleKeydown);
    return () => document.removeEventListener('keydown', handleKeydown);
  });
</script>

<div class="app">
  <!-- Sidebar -->
  <Sidebar
    {activeView}
    onNavigate={navigateTo}
    onSettingsClick={() => navigateTo('settings')}
  />

  <!-- Main Content -->
  <main class="main-content">
    {#if activeView === 'home'}
      <HomeView
        recentAlbums={mockAlbums}
        recommendedAlbums={mockAlbums.slice().reverse()}
        newReleases={mockAlbums.slice(2, 6)}
        onAlbumClick={handleAlbumClick}
      />
    {:else if activeView === 'settings'}
      <SettingsView />
    {:else if activeView === 'album' && selectedAlbum}
      <AlbumDetailView
        album={selectedAlbum}
        onBack={goBack}
      />
    {:else if activeView === 'search'}
      <div class="placeholder-view">
        <h1>Search</h1>
        <p>Search functionality coming soon...</p>
      </div>
    {:else if activeView === 'library'}
      <div class="placeholder-view">
        <h1>Library</h1>
        <p>Your library will appear here...</p>
      </div>
    {/if}
  </main>

  <!-- Now Playing Bar -->
  <NowPlayingBar
    artwork={currentTrack.artwork}
    trackTitle={currentTrack.title}
    artist={currentTrack.artist}
    {isPlaying}
    onTogglePlay={togglePlay}
    {currentTime}
    {duration}
    onSeek={handleSeek}
    {volume}
    onVolumeChange={handleVolumeChange}
    {isShuffle}
    onToggleShuffle={toggleShuffle}
    {repeatMode}
    onToggleRepeat={toggleRepeat}
    {isFavorite}
    onToggleFavorite={toggleFavorite}
    onOpenQueue={() => (isQueueOpen = true)}
    onOpenFullScreen={() => (isFullScreenOpen = true)}
  />

  <!-- Queue Panel -->
  <QueuePanel
    isOpen={isQueueOpen}
    onClose={() => (isQueueOpen = false)}
    {currentTrack}
    {upcomingTracks}
    onClearQueue={() => showToast('Queue cleared', 'info')}
    onSaveAsPlaylist={() => showToast('Saved as playlist', 'success')}
  />

  <!-- Full Screen Now Playing -->
  <FullScreenNowPlaying
    isOpen={isFullScreenOpen}
    onClose={() => (isFullScreenOpen = false)}
    artwork={currentTrack.artwork}
    trackTitle={currentTrack.title}
    artist={currentTrack.artist}
    album="Midnight Dreams"
    quality="24-Bit / 96 kHz"
    qualityLevel={4}
    {isPlaying}
    onTogglePlay={togglePlay}
    {currentTime}
    {duration}
    onSeek={handleSeek}
    {volume}
    onVolumeChange={handleVolumeChange}
    {isShuffle}
    onToggleShuffle={toggleShuffle}
    {repeatMode}
    onToggleRepeat={toggleRepeat}
    {isFavorite}
    onToggleFavorite={toggleFavorite}
    onOpenQueue={() => {
      isFullScreenOpen = false;
      isQueueOpen = true;
    }}
    onOpenFocusMode={() => {
      isFullScreenOpen = false;
      isFocusModeOpen = true;
    }}
  />

  <!-- Focus Mode -->
  <FocusMode
    isOpen={isFocusModeOpen}
    onClose={() => (isFocusModeOpen = false)}
    artwork={currentTrack.artwork}
    trackTitle={currentTrack.title}
    artist={currentTrack.artist}
    {isPlaying}
    onTogglePlay={togglePlay}
    {currentTime}
    {duration}
    onSeek={handleSeek}
  />

  <!-- Toast -->
  {#if toast}
    <Toast
      message={toast.message}
      type={toast.type}
      onClose={hideToast}
    />
  {/if}
</div>

<style>
  .app {
    display: flex;
    height: 100vh;
    overflow: hidden;
    background-color: var(--bg-primary);
  }

  .main-content {
    flex: 1;
    margin-left: 240px;
    margin-bottom: 80px;
    overflow-y: auto;
    padding: 24px 32px;
  }

  .placeholder-view {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--text-muted);
  }

  .placeholder-view h1 {
    font-size: 32px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 8px;
  }

  .placeholder-view p {
    font-size: 16px;
  }
</style>
