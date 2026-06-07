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

## Enterprise Web

- Rust owns routing, static rendering, content indexing, API handlers, and cache invalidation.
- Plain files remain canonical for content and configuration.
- Indexes and caches are derived artifacts.
- Deployment should remain compatible with Headquarters-managed source checkouts when used for websites.

## Source File Standards

Theurgy can ingest wizardry-style HTML and site files, but it should not force that format when a component-oriented format is more maintainable. The target is a file format that is:

- readable without proprietary tooling
- compilable by a free software Rust pipeline
- compatible with static rendering
- expressive enough for component reuse
- deterministic enough for reproducible output

