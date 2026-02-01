// See https://svelte.dev/docs/kit/types#app.d.ts

declare global {
  namespace App {
    // interface Error {}
    // interface Locals {}
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}
  }
}

// Extend Vite's ImportMetaEnv with our custom env variables
interface ImportMetaEnv {
  readonly VITE_BUILD_DATE: string;
  readonly VITE_IMMERSIVE_ENABLED: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}

export {};
