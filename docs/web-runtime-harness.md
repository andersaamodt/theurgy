# Web Runtime Harness

The Theurgy web runtime harness is the first compiled boundary for Wizardry
websites that outgrow shell-first request handling.

## Purpose

- Keep Web Wizardry and Headquarters in charge of install, publish, preview,
  checks, and maintenance.
- Keep canonical site truth in plain files such as `site.conf`, source content,
  and app-owned user-local state roots outside the source checkout.
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
- `replay-theurgy-cgi-fixture` replays captured `env.tsv`, `query.txt`, and
  `body.txt` fixtures against a runtime command with a hard argv boundary.
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
spells, Deployments actuation, file-first state, or site-local configuration.

## Conversion Lessons

Use the Desk and Gazeta conversions as the baseline pattern for future
Wizardry-site ports:

- Convert public reads first. They prove request normalization, file path
  resolution, response compatibility, cache use, and deploy-time runtime
  installation without risking payments, publishing, admin writes, or chat
  controls.
- Keep each runtime action surface narrow. A site may have one public read
  binary, one Nostr read binary, one commerce read binary, and later separate
  mutation binaries. Do not grow one monolithic all-site backend by accident.
- Keep the CGI names stable while changing their bodies. Compatibility wrappers
  should parse the old request shape, set explicit environment variables or a
  normalized request envelope, and exec the compiled runtime.
- Capture replay fixtures before each route switch. The fixture should include
  CGI environment, query string, request body, expected response shape, and
  relevant file-state before/after checks for mutations.
- Build for the deployment host, not only the development laptop. Confirm the
  server Rust/Cargo versions, pin or vendor dependencies when needed, and avoid
  proc-macro or registry-heavy dependency trees on low-resource servers unless
  there is a clear benefit.
- Treat generated artifacts as publish hazards. `target/`, dependency caches,
  local build stamps, and generated output should be ignored or produced by the
  normal deployment hook, not committed as source truth.
- Verify the active release. A successful source patch is not enough; check the
  watched checkout, managed publish status, promoted release path, binary type,
  absence of obsolete interpreted backends, and live browser behavior.
