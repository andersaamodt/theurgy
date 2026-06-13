# Theurgy Rolling Audit

## Round 1
- Date: 2026-06-13
- Auditor scope: repo-grounded audit of Theurgy source, scaffolding defaults, AI-facing docs, tests, and README
- Criteria source: Wizardry core standards plus the rolling Wizardry-family audit criteria for storage, plain-text formats, documentation, and category-first exception handling

## Architecture Observed
- Theurgy is not a Wizardry app; it is the Rust-based escalation layer for Wizardry-family native desktop and enterprise web projects.
- The public boundary is shell spells plus docs, with Rust providing the compiled runtime and scaffold engine.
- The repo explicitly positions Rust, platform adapters, publish tooling, and typed runtime contracts as quarantined exceptions that should not leak back into pure Wizardry or Wizardry Apps.

## Result
- Pass: no

## What Passes
- Theurgy is explicit about its role as an opt-in escalation boundary rather than a replacement for Wizardry: `README.md:1-20`, `.github/AI_DOCS.md:12-19`
- The repo does document its compromise set and quarantine boundary in a way auditors can actually find: `README.md:92-134`, `.github/THEURGY_ETHOS.md`, `docs/platform-quarantine.md`
- The install root follows the Wizardry-family default app-home style: `README.md:78`
- `spells/assay-theurgy` passed in this audit and reported:
  - `theurgy=ok`
  - `runtime=rust`
  - `wizardry_relation=extension-not-replacement`
  - `file_first_default=yes`
  - `database_default=no`
- The repo does have `.tests/` coverage for release adapters: `.tests/release/test-app-publish-release-tools.sh`, `.tests/release/test-ios-promotion-script.sh`

## Findings

### 1. Generated project defaults still normalize repo-local runtime-state directories
- Severity: high
- Why it matters:
  - Current Wizardry-family standards push durable and runtime state out of the source checkout by default.
  - Theurgy-generated projects still normalize checkout-local state paths such as `.theurgy-state/` and `.sitedata/<site>`, which weakens that standard at the scaffold layer.
- Evidence:
  - generated `.gitignore` includes repo-local runtime state: `src/main.rs:1911-1914`
  - generated web manifest sets `canonical_state = ".sitedata/<site>"`: `src/main.rs:2026-2029`
  - generated AI docs only say to keep runtime state out of Git, not out of the checkout: `src/main.rs:2032-2035`
  - README storage section still allows ignored in-checkout runtime state: `README.md:154-156`
- Assessment:
  - The repo understands the problem well enough to ignore these paths.
  - The scaffold defaults still teach a weaker habit than the current Wizardry direction.

### 2. User-edited project contracts default to TOML and JSON instead of the preferred YAML-plus-Markdown family
- Severity: medium
- Why it matters:
  - The rolling audit now prefers YAML-plus-Markdown conventions for durable files users are likely to inspect or edit directly.
  - Theurgy currently teaches new projects through `theurgy.project.toml`, `theurgy.web.toml`, and multiple `.json` IR/manifests that are meant to be first-class edited contracts.
- Evidence:
  - generated project manifest is TOML: `src/main.rs:1918`, `src/main.rs:2019-2029`
  - README tells users to work through `product.ir.json`, `desktop.surface.ir.json`, and `runtime.manifest.json`: `README.md:69`
  - desktop docs keep the same JSON/TOML contract posture: `docs/desktop.md:21-44`
- Assessment:
  - JSON is defensible for machine-facing ABI envelopes.
  - The stronger drift is that primary human-owned project contracts are still normalized around TOML and JSON rather than YAML-oriented text.

### 3. The core validation suite is not currently green
- Severity: high
- Why it matters:
  - Theurgy presents typed runtime validation and generated-runtime contract enforcement as a core value.
  - A failing validation suite means the repo is not currently proving the contract discipline it claims.
- Evidence:
  - `sh scripts/theurgy-cargo test` failed in this audit
  - 23 `src/lib.rs` tests passed, but 11 `theurgy-runtime` tests failed
  - failing examples included:
    - `tests::action_ir_validation_uses_typed_action_contract`
    - `tests::compile_native_mobile_outputs_runtime_contract`
    - `tests::compile_macos_emits_full_runtime_bridge`
    - `tests::compile_app_uses_declared_runtime_manifest_and_surface`
    - `tests::product_ir_validation_uses_structured_json_types`
    - `tests::runtime_manifest_validation_requires_string_arrays`
- Assessment:
  - This is a real release-readiness problem, not just an auditability issue.
  - The failures are concentrated in exactly the contract/runtime areas the repo claims are settled.

## Notes On Storage And Truth
- Theurgy’s stated philosophy is still strongly aligned with Wizardry:
  - file-first truth by default
  - databases optional
  - runtime state out of Git
- The current problem is that generated defaults and some docs are weaker than that philosophy, not that the philosophy is missing.

## Suggested Next Moves
1. Change generated Theurgy project defaults so runtime state lives outside the checkout by default, preferably in explicit user-local state roots.
2. Decide which Theurgy contracts should remain JSON for ABI reasons and which human-owned project contracts should move toward YAML-oriented text.
3. Make `sh scripts/theurgy-cargo test` green again before treating the runtime/compiler contract layer as stable.

## Validation Notes
- `spells/assay-theurgy` passed in this audit.
- `sh scripts/theurgy-cargo test` failed with 11 `theurgy-runtime` test failures.
- This round was a repo-and-contract audit, not a generated-project smoke-build pass.
