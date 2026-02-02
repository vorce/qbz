export type SearchTab = 'all' | 'albums' | 'tracks' | 'artists' | 'playlists';

// Search filter types supported by Qobuz API
export type SearchFilterType = 'MainArtist' | 'Performer' | 'Composer' | 'Label' | 'ReleaseName' | null;

export interface SearchResults<T> {
  items: T[];
  total: number;
  offset: number;
  limit: number;
}

// Most popular item from Qobuz catalog search
export type MostPopularItem<Album, Track, Artist> =
  | { type: 'albums'; content: Album }
  | { type: 'tracks'; content: Track }
  | { type: 'artists'; content: Artist };

export interface Playlist {
  id: number;
  name: string;
  description?: string;
  owner: { id: number; name: string };
  images?: string[];
  images150?: string[];
  images300?: string[];
  tracks_count: number;
  duration: number;
  is_public: boolean;
  genres?: { id: number; name: string; slug?: string }[];
  slug?: string;
  users_count?: number;
}

export interface SearchAllResults<Album, Track, Artist> {
  albums: SearchResults<Album>;
  tracks: SearchResults<Track>;
  artists: SearchResults<Artist>;
  playlists: SearchResults<Playlist>;
  most_popular: MostPopularItem<Album, Track, Artist> | null;
}

export interface SearchState<Album, Track, Artist> {
  query: string;
  activeTab: SearchTab;
  filterType: SearchFilterType;
  albumResults: SearchResults<Album> | null;
  trackResults: SearchResults<Track> | null;
  artistResults: SearchResults<Artist> | null;
  playlistResults: SearchResults<Playlist> | null;
  allResults: SearchAllResults<Album, Track, Artist> | null;
}

let searchState: SearchState<unknown, unknown, unknown> = {
  query: '',
  activeTab: 'all',
  filterType: null,
  albumResults: null,
  trackResults: null,
  artistResults: null,
  playlistResults: null,
  allResults: null,
};

export function getSearchState<Album, Track, Artist>(): SearchState<Album, Track, Artist> {
  return searchState as SearchState<Album, Track, Artist>;
}

export function setSearchState<Album, Track, Artist>(next: SearchState<Album, Track, Artist>): void {
  const prevQuery = searchState.query;
  searchState = next as SearchState<unknown, unknown, unknown>;
  // Only notify query listeners when query actually changes
  if (prevQuery !== searchState.query) {
    queryListeners.forEach(fn => fn(searchState.query));
  }
}

// Signal to trigger scroll-to-top and focus on search input
let focusTrigger = 0;
const focusListeners: Set<() => void> = new Set();

export function triggerSearchFocus(): void {
  focusTrigger++;
  focusListeners.forEach(fn => fn());
}

export function subscribeSearchFocus(callback: () => void): () => void {
  focusListeners.add(callback);
  return () => focusListeners.delete(callback);
}

// Query synchronization for sidebar input
const queryListeners: Set<(query: string) => void> = new Set();

export function getSearchQuery(): string {
  return searchState.query;
}

export function setSearchQuery(query: string): void {
  if (searchState.query === query) return; // No change, don't notify
  searchState.query = query;
  queryListeners.forEach(fn => fn(query));
}

export function subscribeSearchQuery(callback: (query: string) => void): () => void {
  queryListeners.add(callback);
  return () => queryListeners.delete(callback);
}

export function clearSearchState(): void {
  searchState = {
    query: '',
    activeTab: 'all',
    filterType: null,
    albumResults: null,
    trackResults: null,
    artistResults: null,
    playlistResults: null,
    allResults: null,
  };
  queryListeners.forEach(fn => fn(''));
}
