# Platform Quarantine

Theurgy is the quarantine layer for professional platform machinery that does not belong in pure wizardry.

## Rule

- Keep wizardry POSIX-first, shell-first, and minimal.
- Put native desktop platform machinery in theurgy.
- Expose platform machinery through theurgy spells and typed runtime contracts.
- Keep generated native source reproducible from plain manifests.
- Keep platform-specific adapters thin over shared theurgy runtime state.

## Apple And macOS

Apple-specific machinery belongs in theurgy adapters, not in pure wizardry:

- Swift and SwiftUI source generation
- `.app` bundle metadata
- `Info.plist` generation
- launch lifecycle hooks
- menu bar and Dock behavior
- code signing and notarization support
- app verification, app-store policing, review gates, and publish-key workflows
- Xcode-shaped project output when unavoidable

Theurgy may generate or manage these files, but the durable project truth should remain a plain theurgy manifest plus ordinary source files.

## App Publishing

App publishing machinery belongs in theurgy when it touches store review, platform verification, notarization, signing credentials, App Store Connect, TestFlight, Google Play, or equivalent platform-policing systems.

Wizardry-facing repositories may keep thin compatibility wrappers at old paths so existing CI and menus keep working, but the implementation should live in theurgy. The wrapper boundary is the compromise: it preserves ordinary spell/script ergonomics while preventing platform-store assumptions from becoming part of the pure Wizardry Apps implementation layer.

## Free And Open Preference

When a platform gives a closed or proprietary default path, theurgy should standardize the least enclosed workable alternative:

- prefer command-line build flows over IDE-only workflows
- prefer generated source over hand-maintained platform boilerplate
- prefer free software adapters on Linux and other open platforms
- document unavoidable proprietary platform steps as compromises

## Wizardry Boundary

Wizardry or wizardry-apps spells that need this machinery should cross the boundary through `invoke-theurgy`, then call theurgy spells by name. They should not embed Apple, Cargo, native bundle, Rust runtime, app verification, app-store policing, publish-key, or web-runtime details directly.
