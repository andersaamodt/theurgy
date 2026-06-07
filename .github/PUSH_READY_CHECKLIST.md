# Push-Ready Checklist

- Worktree is clean before stopping.
- Source, fixtures, docs, and reproducible config may be tracked.
- Runtime state, logs, screenshots, compiled binaries, caches, and scratch output must stay ignored or outside the repo.
- Generated files may be tracked only when they are canonical, reproducible, and documented.
- New durable storage paths must be documented in `README.md` or `.github/`.
- No secrets, tokens, personal notes, or disposable local paths belong in tracked files.
- Run `cargo fmt --check` and `cargo test` before committing Rust changes.

