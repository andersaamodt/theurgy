# Mobile Browser Proof Quarantine

Theurgy owns repeatable proof patterns that touch closed or host-specific mobile browser machinery. Keep app-specific login, storage, and UI logic in the app; move the reusable simulator, browser, signer, and evidence workflow knowledge here.

## Boundary

- Keep website behavior normal: browser pages, links, forms, storage policy, and auth logic remain in the website source.
- Put Xcode Simulator, Safari extension activation, mobile browser handoff quirks, adb cages, and screenshot evidence capture in theurgy or wizardry-backed proof helpers.
- Treat operator approval as the security property under test; do not fake signer approval or browser permission prompts in an affirmative proof.
- Use temp or user-local proof roots for servers, logs, screenshots, cloned signer sources, and mobile build products; never write proof output into the app checkout.
- Prefer least-persistent permissions during proof, such as Safari's one-day extension-origin grant, unless the user explicitly asks for durable local setup.

## iOS Safari Extension Proofs

- A simulator-installable NIP-07 Safari extension is a valid enterprise-website signer path; it does not imply a native desktop app target.
- The reusable flow is: install or build the simulator app, install it with `xcrun simctl install`, launch it once if needed, enable its Safari extension, grant the proof origin, open the website proof page, trigger the real website login control, approve the signer prompt, and capture a simulator screenshot.
- Use `xcrun simctl list devices booted` to verify the target, `xcrun simctl openurl` to open the proof page, and `xcrun simctl io booted screenshot` for evidence.
- Safari extension enablement is a closed-source UI surface; if no stable command-line API exists, record the exact UI path and keep the interaction scoped to the simulator.
- On iOS Safari, distinguish extension installation from extension availability: an installed extension may still be off, missing origin permission, or not injected until Safari is relaunched.
- For NIP-07 signers, proof success is `window.nostr.getPublicKey()` returning through the real site control and the site showing the connected public key.

## Android Browser And Signer Proofs

- Android proof helpers must invoke adb through Wizardry's caged wrapper such as `firewalled-adb`, or through an explicitly equivalent adb shim; do not let project scripts silently fall back to unsafe raw network device access when a cage is available.
- If `PIEPLATE_ADB`, `ADB`, or an equivalent override points at `firewalled-adb`, wrapper detection must use the wrapper calling convention instead of raw `adb -s`.
- Wireless Debugging is an operator-managed prerequisite; scripts should report missing, unauthorized, or offline devices without trying to reset the phone.
- Separate “Android resolves the app scheme” from “the browser dispatches the page-launched scheme.” A direct `am start -a android.intent.action.VIEW -d nostrconnect://...` success proves the signer resolver, not the browser handoff.
- For browser-launched external protocols, prove the actual path from the website tap, then capture foreground package evidence. If focus stays in Firefox or another browser, record that as a browser dispatch blocker rather than signer failure.
- Do not reset, clear, or kill the user's normal browser profile while debugging a mobile proof unless the user explicitly authorizes it.

## Mobile Signer Proof Vocabulary

- `signer-installed`: the signer app or extension bundle exists on the device or simulator.
- `signer-resolves`: the platform can route a direct custom scheme or extension request to the signer.
- `browser-dispatches`: the mobile browser hands off a user-initiated website action to the signer.
- `approval-completed`: the signer prompt was approved by a user/operator, not simulated.
- `site-connected`: the website returned from the signer path and rendered the connected identity.
- An affirmative end-to-end proof requires `browser-dispatches`, `approval-completed`, and `site-connected`; lower states are useful diagnostics but not full login success.

## Evidence Pattern

- Print a machine-readable status line and schema name from proof scripts.
- Include the platform, browser package or simulator target, signer identifier, proof URL, and evidence screenshot path.
- Capture UI state after failures as well as successes; failed handoff screenshots are reusable quarantine knowledge.
- Keep evidence in `${TMPDIR:-/tmp}` or an explicit external evidence directory.
- Make proof gates opt-in or operator-confirmed by default, because mobile devices, signers, and browser prompts are live local state.
- Install Playwright-managed browser binaries through `spells/install-theurgy-browser-proof-runtime` so desktop browser target checks stay repeatable and quarantined outside app repositories; do not use Homebrew for this dependency.
