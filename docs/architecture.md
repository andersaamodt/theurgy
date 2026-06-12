# Architecture

Theurgy is a higher-integration layer for wizardry-family apps.

## Runtime Split

- wizardry remains best for menus, one-shot spells, installation helpers, and inspectable shell workflows.
- theurgy owns latency-sensitive runtimes, native app control planes, web request paths, indexing, and state hydration.
- shell remains available at the boundary, but hot paths do not spawn shell pipelines for every small action.

## Native Desktop

- Rust owns manifest loading, command dispatch, state snapshots, indexing, and IPC.
- Platform UI adapters should receive structured snapshots and send typed actions.
- macOS can target SwiftUI, Linux can target GTK, and future adapters can be added without changing the core state model.
- Backend actions are allowlisted and explicit.

## Product Runtime And Compiler

- Theurgy owns the opt-in Product IR for apps that need a shared typed app brain across native targets.
- Product IR describes identity, domain objects, state snapshots, typed actions, persistence roots, background jobs, audit behavior, and release targets.
- Desktop Surface IR and Mobile Surface IR are separate projections from Product IR; do not force one universal widget tree across desktop and mobile.
- Native adapters should be platform-owned and thin over the runtime protocol: `stateCommand`, `statusCommand`, `subscribeStatusCommand`, `operationStatusCommand`, `actionCommand`, `historyCommand`, and operation progress records.
- Generated runtime metadata must declare the adapter transport boundary. Desktop adapters use `local-process-json`; mobile adapters use `external-json-abi` until generated Swift/Kotlin bindings or an equivalent mobile-safe bridge replaces the JSON ABI.
- Desktop generated runtimes must expose `requestCommand` and `requestCommandManifest` for the local typed request dispatcher; mobile generated runtimes must omit local request commands and stay on external JSON ABI handoff.
- Mobile JSON ABI handoffs must carry Theurgy-owned request schema IDs for state, status, subscribe-status, action, operation-status, and operation-history requests, so platform hosts are not coupled to undocumented envelope conventions.
- The runtime CLI must also accept those typed request envelopes through `run-request`, so adapters and test harnesses can exercise one protocol instead of re-encoding command-specific arguments.
- Long-running Product IR actions require a typed operation status bridge in generated runtimes so progress and terminal state are inspectable without platform-specific polling conventions.
- Existing wizardry-apps shell-first builds remain valid without theurgy; theurgy is the higher-integration path, not a mandatory dependency for ordinary script-first apps.

## Enterprise Web

- Rust owns routing, static rendering, content indexing, API handlers, and cache invalidation.
- Plain files remain canonical for content and configuration.
- Indexes and caches are derived artifacts.
- Deployment should remain compatible with Headquarters-managed source checkouts when used for websites.
- The first runtime boundary is a harness, not a framework takeover: Web Wizardry keeps lifecycle flows, the web server remains an adapter, and Rust normalizes requests before site modules replace hot shell paths.
- Prefer integrating existing free software components such as axum, Rust FastCGI adapters, serde, tera, and tantivy over implementing basic routing, serialization, templating, or search engines from scratch.

## Source File Standards

Theurgy can ingest wizardry-style HTML and site files, but it should not force that format when a component-oriented format is more maintainable. The target is a file format that is:

- readable without proprietary tooling
- compilable by a free software Rust pipeline
- compatible with static rendering
- expressive enough for component reuse
- deterministic enough for reproducible output
