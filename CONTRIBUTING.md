# Contributing to QBZ

This project is actively evolving. Contributions are welcome, but we have a few rules to keep releases stable and avoid regressions (especially around audio output).

## Quick rules

- Write clear, concise English (no emojis in code, comments, or commit messages).
- Keep PRs focused and small when possible.
- Do not change app branding or legal disclaimers without discussing it first.
- Do not modify protected audio-backend behavior unless explicitly requested by the maintainer.

## Branch naming

We use a consistent branch naming scheme:

`<type>/<origin>/<branch_name>`

- `type`: `feature` | `bugfix` | `hotfix` | `refactor` | `release` | `chore` | `docs`
- `origin`:
  - `internal`: created/owned by maintainers
  - `external`: branches/commits authored by third-party contributors (PRs)

Examples:

- `feature/internal/offline-cache-encryption`
- `bugfix/internal/login-footer-alignment`
- `docs/internal/contributing-process`
- `feature/external/add-album-to-playlist`

## Branch workflow

We use a **pre-release integration branch** to keep `main` stable and release-ready at all times.

```
feature/xyz ──┐
bugfix/abc  ──┼──> pre-release ──> main (tagged release)
hotfix/123  ──┘
```

### Branch hierarchy

1. **`main`** - Releases ONLY. Protected branch. Merging here triggers a tagged release.
2. **`pre-release`** - Integration branch. All features and fixes merge here first.
3. **`feature/*`, `bugfix/*`, etc.** - Individual work branches.

### For contributors

**All PRs must target `pre-release`, not `main`.**

PRs targeting `main` will be closed and asked to retarget to `pre-release`.

### Procedure (maintainer)

1. **Triage**
   - Confirm scope and that it does not touch protected areas (audio routing/backends, credential storage, etc.) unless requested.
   - Verify PR targets `pre-release` (not `main`).
2. **Check out the PR**
   - `gh pr checkout <PR_NUMBER>`
3. **Rename the checked-out branch (local)**
   - Use an `external` branch name so it's obvious these commits are third-party authored:
   - `git branch -m <type>/external/<topic>`
4. **Merge to pre-release**
   - `git checkout pre-release`
   - `git merge --no-ff <type>/external/<topic>`
5. **Run checks**
   - Frontend: `npm run build`
   - Backend (when Rust changes): `cargo check` (run from `src-tauri/`)
6. **Push pre-release**
   - `git push origin pre-release`
7. **Close the PR with a comment** explaining it was merged to `pre-release`.

### Releasing to main

When ready to release:

```bash
git checkout main
git merge pre-release
git push origin main
git tag vX.Y.Z
git push origin vX.Y.Z
```

This is done exclusively by maintainers.

### Merge strategy note (to preserve “external” authorship)

If you want the git history to clearly show third-party authored commits, avoid “squash merge”.
Prefer:

- **Create a merge commit**, or
- **Rebase and merge** (preserves individual commits/authors)

## What to include in PRs

- A short description of the problem and solution.
- Screenshots for UI changes when possible.
- Notes about any breaking changes or migrations.

## What not to include

- Large refactors mixed with feature work.
- Changes that reintroduce removed UI/UX patterns (for example, exporting offline cache files).

---

## Internationalization (i18n)

**All UI text must use the translation system.** No hardcoded strings in Svelte templates.

### Locale Files

Translations are stored in:
- `src/lib/i18n/locales/en.json` (English)
- `src/lib/i18n/locales/es.json` (Spanish)

### Before Adding New Text

1. **Read the existing locale files first** to check if a translation already exists
2. **Reuse existing keys** - avoid duplicating translations
3. **Add to both files** - every new key must exist in en.json AND es.json

### How to Use Translations

Import and use the `t` store:

```svelte
<script lang="ts">
  import { t } from '$lib/i18n';
</script>

<!-- In templates -->
<button>{$t('actions.save')}</button>
<span>{$t('settings.audio.title')}</span>
```

### Interpolation Format

svelte-i18n requires a specific format for variable interpolation:

```typescript
// ❌ WRONG - will show {name} literally
$t('greeting', { name: userName })

// ✅ CORRECT - wrap in values object
$t('greeting', { values: { name: userName } })
```

### Translation Key Naming

Use dot notation for namespacing:

```json
{
  "actions": {
    "save": "Save",
    "cancel": "Cancel",
    "delete": "Delete"
  },
  "settings": {
    "audio": {
      "title": "Audio",
      "streamingQuality": "Streaming Quality"
    }
  }
}
```

Common top-level sections:
- `actions` - Buttons and common actions
- `toast` - Toast notification messages
- `settings` - Settings view labels
- `library` - Local library related text
- `player` - Player controls and status
- `errors` - Error messages
- `empty` - Empty state messages

### Using Translations in Script Context

The `$t()` store can only be used in reactive contexts. In Svelte 5:

```typescript
// ❌ WRONG - breaks Svelte preprocessing
const label = $t('some.key');

// ✅ CORRECT - use $derived
const label = $derived($t('some.key'));
```

### Checklist for PRs with UI Text

- [ ] No hardcoded strings in Svelte templates
- [ ] New translation keys added to both en.json and es.json
- [ ] Variable interpolation uses `{ values: { ... } }` format
- [ ] Checked existing keys before creating new ones (no duplicates)
- [ ] Keys follow naming conventions
