# Theurgy AI Docs

## Read Order
- Read `README.md` for the public stance.
- Read `.github/THEURGY_ETHOS.md` for project policy.
- Read `.github/PUSH_READY_CHECKLIST.md` before adding paths or generated output.
- Read `docs/architecture.md` before changing runtime boundaries.
- Read `docs/databases.md` before introducing database-backed behavior.

## Canonical Position
- Theurgy extends wizardry; it does not replace wizardry.
- Use Rust for integrated runtime work that would otherwise fan out through many shell processes.
- Keep plain files as durable truth unless a documented transaction or scale requirement justifies more.
- Keep Wizardry-facing names lowercase unless quoting a formal project name.
- Generated blank projects use AGPL-3.0-or-later plus Wizardry Addendum.
- Theurgy repository code is OWL 3.1 unless a file explicitly says otherwise.

## Validation
- Run `cargo fmt --check`.
- Run `cargo test`.
- Run `spells/assay-theurgy`.
- Run at least one conjure spell into a temporary directory and inspect it with `spells/inspect-theurgy-project`.
