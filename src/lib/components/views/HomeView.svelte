<script lang="ts">
  import HeroSection from '../HeroSection.svelte';
  import HorizontalScrollRow from '../HorizontalScrollRow.svelte';
  import AlbumCard from '../AlbumCard.svelte';

  interface Album {
    id?: string;
    artwork: string;
    title: string;
    artist: string;
    quality?: string;
  }

  interface Props {
    featuredAlbum?: {
      artwork: string;
      title: string;
      artist: string;
      year: string;
    };
    recentAlbums?: Album[];
    recommendedAlbums?: Album[];
    newReleases?: Album[];
    onAlbumClick?: (albumId: string) => void;
  }

  let { featuredAlbum, recentAlbums = [], recommendedAlbums = [], newReleases = [], onAlbumClick }: Props = $props();

  const defaultFeatured = {
    artwork: 'https://picsum.photos/seed/featured/800/400',
    title: 'Featured Album',
    artist: 'Featured Artist',
    year: '2024'
  };

  const featured = featuredAlbum ?? (recentAlbums[0] ? {
    artwork: recentAlbums[0].artwork,
    title: recentAlbums[0].title,
    artist: recentAlbums[0].artist,
    year: '2024'
  } : defaultFeatured);
</script>

<div class="home-view">
  <!-- Hero/Featured Section -->
  <HeroSection
    artwork={featured.artwork}
    title={featured.title}
    artist={featured.artist}
    year={featured.year}
  />

  <!-- Recently Played -->
  {#if recentAlbums.length > 0}
    <HorizontalScrollRow title="Escuchado recientemente">
      {#snippet children()}
        {#each recentAlbums as album}
          <AlbumCard
            artwork={album.artwork}
            title={album.title}
            artist={album.artist}
            quality={album.quality}
            onclick={() => onAlbumClick?.(album.id ?? '')}
          />
        {/each}
        <div class="spacer"></div>
      {/snippet}
    </HorizontalScrollRow>
  {/if}

  <!-- Recommended For You -->
  {#if recommendedAlbums.length > 0}
    <HorizontalScrollRow title="Recomendado para ti">
      {#snippet children()}
        {#each recommendedAlbums as album}
          <AlbumCard
            artwork={album.artwork}
            title={album.title}
            artist={album.artist}
            quality={album.quality}
            size="large"
            onclick={() => onAlbumClick?.(album.id ?? '')}
          />
        {/each}
        <div class="spacer"></div>
      {/snippet}
    </HorizontalScrollRow>
  {/if}

  <!-- New Releases -->
  {#if newReleases.length > 0}
    <HorizontalScrollRow title="Nuevos lanzamientos">
      {#snippet children()}
        {#each newReleases as album}
          <AlbumCard
            artwork={album.artwork}
            title={album.title}
            artist={album.artist}
            quality={album.quality}
            onclick={() => onAlbumClick?.(album.id ?? '')}
          />
        {/each}
        <div class="spacer"></div>
      {/snippet}
    </HorizontalScrollRow>
  {/if}
</div>

<style>
  .home-view {
    width: 100%;
  }

  .spacer {
    width: 60px;
    flex-shrink: 0;
  }
</style>
