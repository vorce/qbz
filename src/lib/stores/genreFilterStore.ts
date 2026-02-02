/**
 * Genre filter store with context support and 3-level hierarchy
 * Each context (home, favorites) has independent persistence
 */

import { invoke } from '@tauri-apps/api/core';

export interface GenreInfo {
  id: number;
  name: string;
  color?: string;
  slug?: string;
  parentId?: number;
}

export interface GenreTreeNode {
  genre: GenreInfo;
  children: GenreTreeNode[];
}

export type GenreFilterContext = 'home' | 'favorites';

interface ContextState {
  selectedGenreIds: Set<number>;
  rememberSelection: boolean;
  listeners: Set<() => void>;
}

interface GenreFilterState {
  parentGenres: GenreInfo[];
  allGenres: GenreInfo[];
  genreTree: GenreTreeNode[];
  childrenByParent: Map<number, GenreInfo[]>;
  isLoading: boolean;
  contexts: Record<GenreFilterContext, ContextState>;
}

const STORAGE_KEYS: Record<GenreFilterContext, string> = {
  home: 'qbz_genre_filter_home',
  favorites: 'qbz_genre_filter_favorites',
};

let currentContext: GenreFilterContext = 'home';

const state: GenreFilterState = {
  parentGenres: [],
  allGenres: [],
  genreTree: [],
  childrenByParent: new Map(),
  isLoading: false,
  contexts: {
    home: {
      selectedGenreIds: new Set(),
      rememberSelection: true,
      listeners: new Set(),
    },
    favorites: {
      selectedGenreIds: new Set(),
      rememberSelection: true,
      listeners: new Set(),
    },
  },
};

function getContextState(context?: GenreFilterContext): ContextState {
  return state.contexts[context ?? currentContext];
}

function notify(context?: GenreFilterContext) {
  const ctx = getContextState(context);
  ctx.listeners.forEach((fn) => fn());
}

function saveToStorage(context?: GenreFilterContext) {
  const ctx = context ?? currentContext;
  const ctxState = getContextState(ctx);
  if (!ctxState.rememberSelection) return;
  try {
    const data = {
      selectedGenreIds: Array.from(ctxState.selectedGenreIds),
      rememberSelection: ctxState.rememberSelection,
    };
    localStorage.setItem(STORAGE_KEYS[ctx], JSON.stringify(data));
  } catch (e) {
    console.error(`Failed to save genre filter state for ${ctx}:`, e);
  }
}

function loadFromStorage(context: GenreFilterContext) {
  try {
    const stored = localStorage.getItem(STORAGE_KEYS[context]);
    if (stored) {
      const data = JSON.parse(stored);
      state.contexts[context].selectedGenreIds = new Set(data.selectedGenreIds || []);
      state.contexts[context].rememberSelection = data.rememberSelection ?? true;
    }
  } catch (e) {
    console.error(`Failed to load genre filter state for ${context}:`, e);
  }
}

export function setContext(context: GenreFilterContext): void {
  currentContext = context;
}

export function getContext(): GenreFilterContext {
  return currentContext;
}

export async function loadGenres(): Promise<void> {
  if (state.parentGenres.length > 0) return;

  state.isLoading = true;
  notify('home');
  notify('favorites');

  try {
    // Fetch top-level genres
    const parentGenres = await invoke<GenreInfo[]>('get_genres', {});

    const childrenByParent = new Map<number, GenreInfo[]>();
    const allGenresList: GenreInfo[] = [...parentGenres];
    const genreTree: GenreTreeNode[] = [];

    // Fetch level 2 and level 3 genres
    for (const parent of parentGenres) {
      const level2Children = await invoke<GenreInfo[]>('get_genres', { parentId: parent.id });
      const taggedLevel2 = level2Children.map(c => ({ ...c, parentId: parent.id }));
      childrenByParent.set(parent.id, taggedLevel2);
      allGenresList.push(...taggedLevel2);

      const parentNode: GenreTreeNode = { genre: parent, children: [] };

      // Fetch level 3 for each level 2
      for (const child of taggedLevel2) {
        const level3Children = await invoke<GenreInfo[]>('get_genres', { parentId: child.id });
        const taggedLevel3 = level3Children.map(gc => ({ ...gc, parentId: child.id }));

        if (taggedLevel3.length > 0) {
          childrenByParent.set(child.id, taggedLevel3);
          allGenresList.push(...taggedLevel3);
        }

        const childNode: GenreTreeNode = {
          genre: child,
          children: taggedLevel3.map(gc => ({ genre: gc, children: [] }))
        };
        parentNode.children.push(childNode);
      }

      genreTree.push(parentNode);
    }

    // Remove duplicates and sort
    const uniqueGenres = Array.from(
      new Map(allGenresList.map(g => [g.id, g])).values()
    );
    uniqueGenres.sort((a, b) => a.name.localeCompare(b.name));
    parentGenres.sort((a, b) => a.name.localeCompare(b.name));
    genreTree.sort((a, b) => a.genre.name.localeCompare(b.genre.name));

    state.parentGenres = parentGenres;
    state.allGenres = uniqueGenres;
    state.genreTree = genreTree;
    state.childrenByParent = childrenByParent;

    // Load saved selections
    loadFromStorage('home');
    loadFromStorage('favorites');

    // Validate saved selections
    const validIds = new Set(uniqueGenres.map((g) => g.id));
    for (const ctx of ['home', 'favorites'] as GenreFilterContext[]) {
      const ctxState = state.contexts[ctx];
      const validSelection = new Set<number>();
      ctxState.selectedGenreIds.forEach((id) => {
        if (validIds.has(id)) {
          validSelection.add(id);
        }
      });
      ctxState.selectedGenreIds = validSelection;
    }
  } catch (e) {
    console.error('Failed to load genres:', e);
    state.parentGenres = [];
    state.allGenres = [];
    state.genreTree = [];
    state.childrenByParent = new Map();
  } finally {
    state.isLoading = false;
    notify('home');
    notify('favorites');
  }
}

export function getGenreFilterState(context?: GenreFilterContext) {
  const ctx = getContextState(context);
  return {
    availableGenres: state.parentGenres,
    allGenres: state.allGenres,
    genreTree: state.genreTree,
    selectedGenreIds: new Set(ctx.selectedGenreIds),
    isLoading: state.isLoading,
    rememberSelection: ctx.rememberSelection,
  };
}

export function getAvailableGenres(): GenreInfo[] {
  return state.parentGenres;
}

export function getAllGenres(): GenreInfo[] {
  return state.allGenres;
}

export function getGenreTree(): GenreTreeNode[] {
  return state.genreTree;
}

export function getChildGenres(parentId: number): GenreInfo[] {
  return state.childrenByParent.get(parentId) || [];
}

/** Get all descendant IDs for a genre (children + grandchildren) */
export function getAllDescendantIds(genreId: number): number[] {
  const descendants: number[] = [];
  const children = state.childrenByParent.get(genreId) || [];

  for (const child of children) {
    descendants.push(child.id);
    // Also get grandchildren
    const grandchildren = state.childrenByParent.get(child.id) || [];
    descendants.push(...grandchildren.map(gc => gc.id));
  }

  return descendants;
}

/** Count total descendants (children + grandchildren) */
export function countDescendants(genreId: number): number {
  return getAllDescendantIds(genreId).length;
}

export function getSelectedGenreIds(context?: GenreFilterContext): Set<number> {
  return new Set(getContextState(context).selectedGenreIds);
}

export function getSelectedGenreId(context?: GenreFilterContext): number | undefined {
  const ids = Array.from(getContextState(context).selectedGenreIds);
  return ids.length === 1 ? ids[0] : undefined;
}

export function getSelectedGenreName(context?: GenreFilterContext): string | undefined {
  const id = getSelectedGenreId(context);
  if (!id) return undefined;
  const genre = state.allGenres.find((g: GenreInfo) => g.id === id);
  return genre?.name;
}

export function getSelectedGenreNames(context?: GenreFilterContext): string[] {
  const ids = Array.from(getContextState(context).selectedGenreIds);
  return ids
    .map(id => state.allGenres.find((g: GenreInfo) => g.id === id)?.name)
    .filter((name): name is string => !!name);
}

/**
 * Get all genre names to filter by, including all descendants of selected genres.
 * Traverses up to 3 levels deep.
 */
export function getFilterGenreNames(context?: GenreFilterContext): string[] {
  const selectedIds = Array.from(getContextState(context).selectedGenreIds);
  const names = new Set<string>();

  for (const id of selectedIds) {
    const genre = state.allGenres.find((g: GenreInfo) => g.id === id);
    if (genre?.name) {
      names.add(genre.name);
    }

    // Add all descendants (children and grandchildren)
    const descendantIds = getAllDescendantIds(id);
    for (const descId of descendantIds) {
      const descGenre = state.allGenres.find((g: GenreInfo) => g.id === descId);
      if (descGenre?.name) {
        names.add(descGenre.name);
      }
    }
  }

  return Array.from(names);
}

export function isGenreSelected(genreId: number, context?: GenreFilterContext): boolean {
  return getContextState(context).selectedGenreIds.has(genreId);
}

export function hasActiveFilter(context?: GenreFilterContext): boolean {
  return getContextState(context).selectedGenreIds.size > 0;
}

export function toggleGenre(genreId: number, context?: GenreFilterContext): void {
  const ctx = context ?? currentContext;
  const ctxState = getContextState(ctx);
  if (ctxState.selectedGenreIds.has(genreId)) {
    ctxState.selectedGenreIds.delete(genreId);
  } else {
    ctxState.selectedGenreIds.add(genreId);
  }
  saveToStorage(ctx);
  notify(ctx);
}

/** Select or deselect multiple genres at once */
export function setGenresSelected(genreIds: number[], selected: boolean, context?: GenreFilterContext): void {
  const ctx = context ?? currentContext;
  const ctxState = getContextState(ctx);

  for (const id of genreIds) {
    if (selected) {
      ctxState.selectedGenreIds.add(id);
    } else {
      ctxState.selectedGenreIds.delete(id);
    }
  }

  saveToStorage(ctx);
  notify(ctx);
}

export function selectGenre(genreId: number, context?: GenreFilterContext): void {
  const ctx = context ?? currentContext;
  const ctxState = getContextState(ctx);
  ctxState.selectedGenreIds.clear();
  ctxState.selectedGenreIds.add(genreId);
  saveToStorage(ctx);
  notify(ctx);
}

export function clearSelection(context?: GenreFilterContext): void {
  const ctx = context ?? currentContext;
  const ctxState = getContextState(ctx);
  ctxState.selectedGenreIds.clear();
  saveToStorage(ctx);
  notify(ctx);
}

export function setRememberSelection(remember: boolean, context?: GenreFilterContext): void {
  const ctx = context ?? currentContext;
  const ctxState = getContextState(ctx);
  ctxState.rememberSelection = remember;
  if (remember) {
    saveToStorage(ctx);
  } else {
    localStorage.removeItem(STORAGE_KEYS[ctx]);
  }
  notify(ctx);
}

export function subscribe(callback: () => void, context?: GenreFilterContext): () => void {
  const ctxState = getContextState(context);
  ctxState.listeners.add(callback);
  return () => ctxState.listeners.delete(callback);
}

// Initialize by loading from storage for all contexts
loadFromStorage('home');
loadFromStorage('favorites');
