import { browser } from '$app/environment';
import { writable } from 'svelte/store';

const STORAGE_KEY = 'qbz-qobuz-tos-accepted';

function readInitial(): boolean {
  if (!browser) return false;
  return localStorage.getItem(STORAGE_KEY) === 'true';
}

export const qobuzTosAccepted = writable<boolean>(readInitial());

if (browser) {
  qobuzTosAccepted.subscribe((value) => {
    localStorage.setItem(STORAGE_KEY, String(value));
  });
}

