# Web Runtime Harness

The Theurgy web runtime harness is the first compiled boundary for Wizardry
websites that outgrow shell-first request handling.

## Purpose

- Keep Web Wizardry and Headquarters in charge of install, publish, preview,
  checks, and maintenance.
- Keep canonical site truth in plain files such as `site.conf`, source content,
  and `.sitedata/<site>/`.
- Move hot request paths into Rust once repeated shell process fan-out becomes
  slow, unreliable, or hard to debug.
- Integrate existing free software instead of inventing basic infrastructure.

## Integrated Components

- `nginx` or `lighttpd` can remain the front door.
- `axum` is the preferred Rust HTTP router when a resident local service is the
  right adapter.
- A Rust FastCGI adapter is the preferred bridge when a site should stay close
  to existing CGI server wiring.
- `serde` owns typed JSON, TOML, and request/response contracts.
- `tera` owns server-side templates when static files are not enough.
- `tantivy` owns search indexes.
- Zola is not part of the Gazeta/Desk-class runtime. It may be useful for
  purely static sites, but it would create special cases for mutable Wizardry
  sites.

## Phase 1 Contract

Phase 1 establishes the shared contract and installable spells:

- `check-theurgy-web-runtime` reports the runtime components and policy.
- `capture-theurgy-cgi-context` converts CGI environment variables into the
  normalized JSON request context.
- `conjure-enterprise-website` writes `theurgy.web.toml` and a
  `cgi/theurgy-cgi-context` compatibility adapter.

This phase does not migrate Gazeta or Desk behavior. It creates the typed
adapter seam that phase 2 can use for the first Desk backend conversion.

## Phase 2 Desk Target

Desk should become the first site module because it is narrow and file-backed:

- parse one action request into a typed Rust command
- load the Desk root and state directory from the Theurgy request context
- take one lock per mutation
- read/write canonical files atomically
- return the existing JSON shape expected by the frontend
- keep a CLI maintenance command backed by the same Rust action engine

## Boundaries

The harness replaces shell in hot request paths. It does not replace Wizardry
spells, Headquarters deployments, file-first state, or site-local configuration.
