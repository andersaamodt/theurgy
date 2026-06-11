# Enterprise Web Track

The enterprise web track is for websites that need professional performance without losing wizardry's convenience.

## Goals

- low-level Rust HTTP and rendering pipeline
- static-first output when possible
- file-first content and config
- derived indexes and caches
- optional database layer only when transactions or query scale require it
- compatible integration with Headquarters-managed source checkouts

## Runtime Harness

The first-phase runtime harness is a typed Rust boundary for Wizardry websites:

- Web Wizardry keeps install, publish, preview, and maintenance flows.
- `nginx` or `lighttpd` can serve static files and forward dynamic routes.
- Rust owns request context normalization, routing adapters, typed JSON
  contracts, indexing, templates, and cache invalidation.
- `axum`, Rust FastCGI adapters, `serde`, `tera`, and `tantivy` are the preferred
  integrated free-software components.
- Zola is not a Gazeta/Desk-class runtime dependency; it is only a possible
  separate fit for purely static sites.

Use `spells/check-theurgy-web-runtime` to inspect the contract and
`spells/capture-theurgy-cgi-context` for the CGI compatibility adapter.

## Initial Scaffold

`spells/conjure-enterprise-website NAME [PATH]` creates:

- `theurgy.project.toml`
- `theurgy.web.toml`
- `cgi/theurgy-cgi-context`
- `content/pages/index.html`
- `src/site.theurgy`
- `.github/AI_DOCS.md`
- `LICENSE`
- `WIZARDRY_ADDENDUM.md`

If `PATH` is omitted, the spell uses `NAME`. The scaffold is not tied to Svelte or another commercial-adjacent framework. Component syntax can evolve, but the runtime commitment is free software, inspectable files, and reproducible rendering.
