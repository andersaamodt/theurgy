# Theurgy Ethos

- Keep wizardry as the transparent prototyping and orchestration layer.
- Expose user workflows as well-named spells, not a catch-all command with subcommands.
- Minimize arguments; default paths should follow the project name when that is clear.
- Move repeated, latency-sensitive, or graph-shaped work into integrated Rust.
- Do not paper over performance problems with one-off exceptions.
- Keep the user-facing workflow convenient without hiding source files, Git history, or state paths.
- Prefer file-first truth, deterministic indexes, and explicit build artifacts.
- Use databases only for transactions, concurrency, query acceleration, or replication needs that files cannot satisfy cleanly.
- Prefer free software, low-level, non-commercially enclosed web stacks.
- Avoid proprietary framework lock-in and commercial platform assumptions.
- Keep bridges explicit: shell, Rust, native UI, and web handlers must have readable contracts.
- Preserve CLI parity for meaningful GUI features.
- For wizardry-native desktop migrations, preserve the shared native schema/generator and make Rust the compiled control plane underneath it instead of replacing the app with a one-platform rewrite.
- Keep shell out of hot interactive paths once a typed Rust runtime exists; leave shell to the surrounding wizardry lifecycle.
- Require release artifacts and tests to prove the compiled core is actually bundled for every supported desktop target.
- Generated macOS apps should feel like normal professional apps: one clean bundle, direct runtime dispatch, explicit signing behavior, and no avoidable Gatekeeper assessment fanout.
- Host-specific firewall and sandbox enforcement belongs here behind clear spell boundaries so Wizardry can offer simple verbs without pretending the host machinery is portable.
