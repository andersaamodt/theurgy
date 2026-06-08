# Enterprise Web Track

The enterprise web track is for websites that need professional performance without losing wizardry's convenience.

## Goals

- low-level Rust HTTP and rendering pipeline
- static-first output when possible
- file-first content and config
- derived indexes and caches
- optional database layer only when transactions or query scale require it
- compatible integration with Headquarters-managed source checkouts

## Initial Scaffold

`spells/conjure-enterprise-website NAME [PATH]` creates:

- `theurgy.project.toml`
- `content/pages/index.html`
- `src/site.theurgy`
- `.github/AI_DOCS.md`
- `LICENSE`
- `WIZARDRY_ADDENDUM.md`

If `PATH` is omitted, the spell uses `NAME`. The scaffold is not tied to Svelte or another commercial-adjacent framework. Component syntax can evolve, but the runtime commitment is free software, inspectable files, and reproducible rendering.
