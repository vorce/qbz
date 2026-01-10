# Plan de Integración: Cache para Offline Listening

## Alcance

Solo implementaremos **cache para offline listening** (no descarga libre a carpetas).
Incluiremos un botón "Open cache folder" que abre la ubicación en el file manager del sistema.

---

## Arquitectura

### Estructura de Archivos

```
~/.cache/qbz-nix/audio/
├── index.db              # SQLite con metadata de tracks cacheados
├── tracks/
│   ├── {track_id}.flac   # Archivos de audio (formato original)
│   ├── {track_id}.flac
│   └── ...
└── artwork/
    ├── {album_id}.jpg    # Artwork para mostrar offline
    └── ...
```

### Base de Datos (SQLite)

```sql
CREATE TABLE cached_tracks (
    id INTEGER PRIMARY KEY,
    track_id INTEGER UNIQUE NOT NULL,
    title TEXT NOT NULL,
    artist TEXT NOT NULL,
    album TEXT,
    album_id TEXT,
    duration_secs INTEGER,
    file_path TEXT NOT NULL,
    file_size_bytes INTEGER NOT NULL,
    format TEXT NOT NULL,           -- 'flac', 'mp3', etc.
    quality TEXT,                   -- 'CD', 'Hi-Res 24/96', etc.
    bit_depth INTEGER,
    sample_rate REAL,
    artwork_path TEXT,
    status TEXT NOT NULL DEFAULT 'downloading', -- 'downloading', 'ready', 'failed'
    progress_percent INTEGER DEFAULT 0,
    error_message TEXT,
    created_at TEXT NOT NULL,
    last_accessed_at TEXT NOT NULL
);

CREATE INDEX idx_track_id ON cached_tracks(track_id);
CREATE INDEX idx_status ON cached_tracks(status);
CREATE INDEX idx_last_accessed ON cached_tracks(last_accessed_at);
```

---

## Módulo Backend: `src-tauri/src/download_cache/`

### Archivos

```
src-tauri/src/download_cache/
├── mod.rs          # Exports y tipos públicos
├── db.rs           # Operaciones SQLite
├── downloader.rs   # Lógica de descarga
└── commands.rs     # Comandos Tauri
```

### Comandos Tauri

```rust
// Iniciar descarga de un track
#[tauri::command]
async fn download_track(track_id: u64, state: State<AppState>) -> Result<(), String>

// Iniciar descarga de un album completo
#[tauri::command]
async fn download_album(album_id: String, state: State<AppState>) -> Result<(), String>

// Iniciar descarga de una playlist
#[tauri::command]
async fn download_playlist(playlist_id: u64, state: State<AppState>) -> Result<(), String>

// Cancelar descarga en progreso
#[tauri::command]
async fn cancel_download(track_id: u64, state: State<AppState>) -> Result<(), String>

// Eliminar track del cache
#[tauri::command]
async fn remove_cached_track(track_id: u64, state: State<AppState>) -> Result<(), String>

// Obtener estado del cache
#[tauri::command]
async fn get_download_cache_stats(state: State<AppState>) -> Result<DownloadCacheStats, String>

// Obtener lista de tracks cacheados
#[tauri::command]
async fn get_cached_tracks(state: State<AppState>) -> Result<Vec<CachedTrackInfo>, String>

// Verificar si un track está cacheado
#[tauri::command]
async fn is_track_cached(track_id: u64, state: State<AppState>) -> Result<bool, String>

// Limpiar todo el cache
#[tauri::command]
async fn clear_download_cache(state: State<AppState>) -> Result<(), String>

// Abrir carpeta de cache en file manager
#[tauri::command]
async fn open_cache_folder() -> Result<(), String>

// Configurar límite de cache
#[tauri::command]
async fn set_cache_limit(limit_mb: Option<u64>, state: State<AppState>) -> Result<(), String>
```

### Tipos de Datos

```rust
#[derive(Debug, Clone, Serialize)]
pub struct CachedTrackInfo {
    pub track_id: u64,
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub duration_secs: u64,
    pub file_size_bytes: u64,
    pub quality: String,
    pub status: DownloadStatus,
    pub progress_percent: u8,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub enum DownloadStatus {
    Queued,
    Downloading,
    Ready,
    Failed,
}

#[derive(Debug, Clone, Serialize)]
pub struct DownloadCacheStats {
    pub total_tracks: usize,
    pub ready_tracks: usize,
    pub downloading_tracks: usize,
    pub total_size_bytes: u64,
    pub limit_bytes: Option<u64>,
    pub cache_path: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DownloadProgress {
    pub track_id: u64,
    pub progress_percent: u8,
    pub bytes_downloaded: u64,
    pub total_bytes: u64,
    pub status: DownloadStatus,
}
```

---

## Flujo de Descarga

### 1. Usuario solicita descarga

```
Usuario -> Click "Download" -> download_track(track_id)
                                    |
                                    v
                            Insertar en DB con status='queued'
                                    |
                                    v
                            Iniciar task de descarga
                                    |
                                    v
                            Emitir evento: download_started
```

### 2. Proceso de descarga

```
Task de descarga:
    1. Obtener stream URL (get_stream_url_with_fallback)
    2. Actualizar status='downloading'
    3. Descargar bytes en chunks
       - Emitir eventos de progreso cada 5%
       - Guardar a archivo temporal
    4. Al completar:
       - Mover archivo a ubicación final
       - Actualizar status='ready'
       - Emitir evento: download_completed
    5. Si falla:
       - Actualizar status='failed' + error_message
       - Emitir evento: download_failed
```

### 3. Playback con cache

```
play_track(track_id):
    1. Verificar si track está en download cache (status='ready')
    2. Si está: reproducir desde archivo local
    3. Si no: streaming normal + cachear en memoria
```

---

## Eventos Frontend (Tauri Events)

```typescript
// Eventos emitidos por el backend
interface DownloadEvents {
  'download:started': { trackId: number };
  'download:progress': { trackId: number; percent: number; bytesDownloaded: number };
  'download:completed': { trackId: number };
  'download:failed': { trackId: number; error: string };
  'download:cancelled': { trackId: number };
}
```

---

## UI Frontend

### Componentes

1. **DownloadButton** - Botón de descarga para tracks/albums
   - Estados: idle, downloading (con %), downloaded, failed
   - Click en downloaded -> opción de remover

2. **DownloadManager** - Panel/Modal de gestión de descargas
   - Lista de descargas en progreso
   - Lista de tracks descargados
   - Botón "Open cache folder"
   - Estadísticas de uso

3. **Settings Section** - En SettingsView
   - Cache limit slider (500MB - 10GB - Unlimited)
   - Current usage display
   - "Clear all downloads" button
   - "Open cache folder" button

### Indicadores Visuales

```
TrackRow:
  [Play] [Title] [Artist] [Duration] [Quality] [♥] [⬇️] [⋮]
                                                 ^
                                           Download indicator:
                                           - No icon: not cached
                                           - ↓ animado: downloading
                                           - ✓ verde: cached
                                           - ⚠️ rojo: failed
```

---

## Política de Eviction

1. **Límite configurable**: Default 2GB, opciones 500MB-10GB-Unlimited
2. **LRU por last_accessed_at**: Al exceder límite, eliminar tracks menos usados
3. **Protección**: No eliminar tracks en reproducción actual o queue
4. **Manual**: Usuario puede eliminar tracks individuales o limpiar todo

---

## Implementación por Fases

### Fase 1: Infraestructura básica
- [ ] Crear módulo `download_cache`
- [ ] Inicializar SQLite DB
- [ ] Comandos básicos: `download_track`, `is_track_cached`, `get_cached_tracks`
- [ ] Integrar con `play_track` para usar cache local

### Fase 2: UI de descarga individual
- [ ] DownloadButton component
- [ ] Indicadores en TrackRow
- [ ] Progress events y actualización en tiempo real

### Fase 3: Descarga por lotes
- [ ] `download_album`, `download_playlist`
- [ ] Cola de descargas con límite de concurrencia (3 simultáneas)
- [ ] DownloadManager panel

### Fase 4: Settings y gestión
- [ ] Cache limit configuration
- [ ] Eviction automático
- [ ] "Open cache folder" button
- [ ] Clear cache functionality

---

## Consideraciones Legales

- **NO** exponemos esto como "descargar música"
- **SÍ** lo presentamos como "disponible offline" (igual que Spotify/Apple Music)
- El botón "Open cache folder" es un easter egg, no una feature documentada
- Los archivos en cache NO tienen metadata de tags (son raw streams)
- Si Qobuz cambia sus ToS, el cache se puede desactivar con un flag

---

## Dependencias Adicionales

```toml
# Ya tenemos
rusqlite = { version = "0.31", features = ["bundled"] }
dirs = "5"

# Podríamos necesitar
tokio-util = "0.7"  # Para streaming de archivos
```

---

## Estimación

- Fase 1: Core functionality
- Fase 2: UI básica
- Fase 3: Batch downloads
- Fase 4: Polish y settings
