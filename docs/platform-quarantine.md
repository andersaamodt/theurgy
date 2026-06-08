# Platform Quarantine

Theurgy is the quarantine layer for professional platform machinery that does not belong in pure wizardry.

## Rule

- Keep wizardry POSIX-first, shell-first, and minimal.
- Put native desktop platform machinery in Theurgy.
- Expose platform machinery through Theurgy spells and typed runtime contracts.
- Keep generated native source reproducible from plain manifests.
- Keep platform-specific adapters thin over shared Theurgy runtime state.

## Apple And macOS

Apple-specific machinery belongs in Theurgy adapters, not in pure wizardry:

- Swift and SwiftUI source generation
- `.app` bundle metadata
- `Info.plist` generation
- launch lifecycle hooks
- menu bar and Dock behavior
- code signing and notarization support
- Xcode-shaped project output when unavoidable

Theurgy may generate or manage these files, but the durable project truth should remain a plain Theurgy manifest plus ordinary source files.

## Free And Open Preference

When a platform gives a closed or proprietary default path, Theurgy should standardize the least enclosed workable alternative:

- prefer command-line build flows over IDE-only workflows
- prefer generated source over hand-maintained platform boilerplate
- prefer free software adapters on Linux and other open platforms
- document unavoidable proprietary platform steps as compromises

## Wizardry Boundary

Wizardry spells that need this machinery should call Theurgy spells by name after `ensure-theurgy` has established the install. They should not embed Apple, Cargo, native bundle, or web-runtime details directly.

