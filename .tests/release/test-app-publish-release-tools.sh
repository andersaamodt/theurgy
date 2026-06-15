#!/bin/sh

set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd -P)
state_home=${XDG_STATE_HOME:-${HOME:-/tmp}/.local/state}
scratch_root=${THEURGY_TEST_SCRATCH_ROOT:-"$state_home/theurgy/test-scratch"}
tmp_dir="$scratch_root/app-publish-release-tools"
rm -rf "$tmp_dir"
mkdir -p "$tmp_dir"
trap 'rm -rf "$tmp_dir"' EXIT HUP INT TERM

for script in \
  build-ios-app.sh \
  promote-ios-release.sh \
  promote-play-track.sh \
  sign-and-notarize-macos.sh \
  upload-play-internal.sh \
  upload-testflight.sh
do
  [ -x "$ROOT_DIR/tools/release/$script" ] || {
    printf '%s\n' "missing executable release adapter: $script" >&2
    exit 1
  }
  sh "$ROOT_DIR/tools/release/$script" --help >/dev/null
done

bad_ios_slug=$(printf 'forge\nforged=1')
if sh "$ROOT_DIR/tools/release/build-ios-app.sh" "$bad_ios_slug" "$tmp_dir/ios-out" smoke >"$tmp_dir/ios-bad-slug.out" 2>"$tmp_dir/ios-bad-slug.err"; then
  printf '%s\n' "build-ios-app accepted newline app slug" >&2
  exit 1
fi
grep -F "invalid app slug" "$tmp_dir/ios-bad-slug.err" >/dev/null
if tr '\r' '\n' <"$tmp_dir/ios-bad-slug.err" | grep -E '^forged=' >/dev/null 2>&1; then
  printf '%s\n' "build-ios-app emitted forged rows from invalid app slug" >&2
  exit 1
fi

bad_ios_out="$tmp_dir/ios-out
forged=1"
if sh "$ROOT_DIR/tools/release/build-ios-app.sh" forge "$bad_ios_out" smoke >"$tmp_dir/ios-bad-out.out" 2>"$tmp_dir/ios-bad-out.err"; then
  printf '%s\n' "build-ios-app accepted newline output directory" >&2
  exit 1
fi
grep -F "output directory must not contain line breaks" "$tmp_dir/ios-bad-out.err" >/dev/null

fake_apps_root="$tmp_dir/wizardry-apps"
mkdir -p "$fake_apps_root/tools/release"
cat >"$fake_apps_root/tools/release/get-app-name.sh" <<'SH'
#!/bin/sh
printf '%s\n' "App Forge"
SH
cat >"$fake_apps_root/tools/release/get-app-bundle-id.sh" <<'SH'
#!/bin/sh
printf '%s\n' "com.example.forge"
SH
chmod +x "$fake_apps_root/tools/release/get-app-name.sh" "$fake_apps_root/tools/release/get-app-bundle-id.sh"
if RELEASE_VERSION='v1.2.3/../../bad' \
   WIZARDRY_APPS_ROOT="$fake_apps_root" \
   sh "$ROOT_DIR/tools/release/build-ios-app.sh" forge "$tmp_dir/ios-version-out" smoke >"$tmp_dir/ios-invalid-version.out" 2>"$tmp_dir/ios-invalid-version.err"; then
  printf '%s\n' "build-ios-app accepted invalid release version" >&2
  exit 1
fi
grep -F "invalid release version" "$tmp_dir/ios-invalid-version.err" >/dev/null

aab_path="$tmp_dir/app.aab"
: > "$aab_path"
bad_aab_path="$tmp_dir/app
forged=1.aab"
: > "$bad_aab_path"
if sh "$ROOT_DIR/tools/release/upload-play-internal.sh" "$bad_aab_path" "com.example.app" internal >"$tmp_dir/play-upload-bad-aab-path.err" 2>&1; then
  printf '%s\n' "upload-play-internal accepted newline AAB path" >&2
  exit 1
fi
grep -F "AAB path must not contain line breaks" "$tmp_dir/play-upload-bad-aab-path.err" >/dev/null

bad_aab_suffix="$tmp_dir/app.txt"
: > "$bad_aab_suffix"
if sh "$ROOT_DIR/tools/release/upload-play-internal.sh" "$bad_aab_suffix" "com.example.app" internal >"$tmp_dir/play-upload-bad-aab-suffix.err" 2>&1; then
  printf '%s\n' "upload-play-internal accepted a non-AAB artifact path" >&2
  exit 1
fi
grep -F "AAB path must end with .aab" "$tmp_dir/play-upload-bad-aab-suffix.err" >/dev/null

if sh "$ROOT_DIR/tools/release/upload-play-internal.sh" "$aab_path" "com.example/../../other" internal >"$tmp_dir/play-upload-invalid-package.err" 2>&1; then
  printf '%s\n' "upload-play-internal accepted invalid package name" >&2
  exit 1
fi
grep -F "invalid package name" "$tmp_dir/play-upload-invalid-package.err" >/dev/null

if PLAY_RELEASE_STATUS='maybe' \
   sh "$ROOT_DIR/tools/release/upload-play-internal.sh" "$aab_path" "com.example.app" internal >"$tmp_dir/play-upload-invalid-status.err" 2>&1; then
  printf '%s\n' "upload-play-internal accepted invalid release status" >&2
  exit 1
fi
grep -F "invalid release status" "$tmp_dir/play-upload-invalid-status.err" >/dev/null

if sh "$ROOT_DIR/tools/release/promote-play-track.sh" "com.example/../../other" internal production >"$tmp_dir/play-promote-invalid-package.err" 2>&1; then
  printf '%s\n' "promote-play-track accepted invalid package name" >&2
  exit 1
fi
grep -F "invalid package name" "$tmp_dir/play-promote-invalid-package.err" >/dev/null

if sh "$ROOT_DIR/tools/release/promote-play-track.sh" "com.example.app" "internal/../../prod" production >"$tmp_dir/play-promote-invalid-track.err" 2>&1; then
  printf '%s\n' "promote-play-track accepted invalid track" >&2
  exit 1
fi
grep -F "invalid track" "$tmp_dir/play-promote-invalid-track.err" >/dev/null

fake_play_bin="$tmp_dir/fake-play-bin"
mkdir -p "$fake_play_bin"
cat >"$fake_play_bin/openssl" <<'SH'
#!/bin/sh
cat
SH
cat >"$fake_play_bin/curl" <<'SH'
#!/bin/sh
url=''
while [ "$#" -gt 0 ]; do
  case "$1" in
    https://*) url=$1 ;;
  esac
  shift
done
case "$url" in
  https://oauth2.googleapis.com/token)
    printf '%s\n' '{"access_token":"token123\nInjected: bad"}'
    ;;
  *)
    printf '%s\n' '{}'
    ;;
esac
SH
chmod +x "$fake_play_bin/openssl" "$fake_play_bin/curl"
good_service_json='{"client_email":"svc@example.iam.gserviceaccount.com","private_key":"key"}'
if PLAY_SERVICE_ACCOUNT_JSON_BASE64="$good_service_json" \
   PATH="$fake_play_bin:$PATH" \
   sh "$ROOT_DIR/tools/release/upload-play-internal.sh" "$aab_path" com.example.app internal >"$tmp_dir/play-upload-bad-token.out" 2>"$tmp_dir/play-upload-bad-token.err"; then
  printf '%s\n' "upload-play-internal accepted invalid API access token" >&2
  exit 1
fi
grep -F "invalid access token from API" "$tmp_dir/play-upload-bad-token.err" >/dev/null

sign_app="$tmp_dir/Test.app"
mkdir -p "$sign_app"
bad_sign_app="$tmp_dir/Bad
forged=1.app"
mkdir -p "$bad_sign_app"
if sh "$ROOT_DIR/tools/release/sign-and-notarize-macos.sh" "$bad_sign_app" >"$tmp_dir/sign-bad-app-path.out" 2>"$tmp_dir/sign-bad-app-path.err"; then
  printf '%s\n' "sign-and-notarize-macos accepted newline app bundle path" >&2
  exit 1
fi
grep -F "app bundle path must not contain line breaks" "$tmp_dir/sign-bad-app-path.err" >/dev/null

sign_not_app="$tmp_dir/NotApp"
mkdir -p "$sign_not_app"
if sh "$ROOT_DIR/tools/release/sign-and-notarize-macos.sh" "$sign_not_app" >"$tmp_dir/sign-not-app.out" 2>"$tmp_dir/sign-not-app.err"; then
  printf '%s\n' "sign-and-notarize-macos accepted a non-.app directory" >&2
  exit 1
fi
grep -F "app bundle path must be a .app bundle" "$tmp_dir/sign-not-app.err" >/dev/null

fake_sign_bin="$tmp_dir/fake-sign-bin"
mkdir -p "$fake_sign_bin"
cat >"$fake_sign_bin/openssl" <<'SH'
#!/bin/sh
cat
SH
cat >"$fake_sign_bin/security" <<'SH'
#!/bin/sh
exit 0
SH
cat >"$fake_sign_bin/codesign" <<'SH'
#!/bin/sh
exit 0
SH
cat >"$fake_sign_bin/xcrun" <<'SH'
#!/bin/sh
exit 0
SH
chmod +x "$fake_sign_bin/openssl" "$fake_sign_bin/security" "$fake_sign_bin/codesign" "$fake_sign_bin/xcrun"
bad_notary_issuer=$(printf '11111111-1111-1111-1111-111111111111\nforged=1')
if APPLE_P12_BASE64='bad' \
   APPLE_P12_PASSWORD='password' \
   APPLE_DEVELOPER_ID_APP='Developer ID Application: Example (TEAM123456)' \
   APPLE_TEAM_ID='TEAM123456' \
   APPLE_NOTARY_KEY_ID='ABC123DEF4' \
   APPLE_NOTARY_ISSUER_ID="$bad_notary_issuer" \
   APPLE_NOTARY_PRIVATE_KEY_BASE64='bad' \
   PATH="$fake_sign_bin:$PATH" \
   sh "$ROOT_DIR/tools/release/sign-and-notarize-macos.sh" "$sign_app" >"$tmp_dir/sign-bad-issuer.out" 2>"$tmp_dir/sign-bad-issuer.err"; then
  printf '%s\n' "sign-and-notarize-macos accepted invalid notary issuer id" >&2
  exit 1
fi
grep -F "invalid Apple notary issuer id" "$tmp_dir/sign-bad-issuer.err" >/dev/null

printf '%s\n' "app publish release tool checks passed"
