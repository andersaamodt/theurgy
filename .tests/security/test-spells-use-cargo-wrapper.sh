#!/bin/sh

set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd -P)

bad_files=$(
  grep -RIlE '(^|[^[:alnum:]_"$./-])cargo[[:space:]]+(run|build)([[:space:]]|$)' "$ROOT_DIR/spells" \
    | while IFS= read -r path; do
        if grep -nE '(^|[^[:alnum:]_"$./-])cargo[[:space:]]+(run|build)([[:space:]]|$)' "$path" \
          | grep -Fv 'scripts/theurgy-cargo' >/dev/null; then
          printf '%s\n' "$path"
        fi
      done
)

if [ -n "$bad_files" ]; then
  printf '%s\n' "spells must use scripts/theurgy-cargo instead of raw cargo:" >&2
  printf '%s\n' "$bad_files" >&2
  exit 1
fi

printf '%s\n' "spell cargo wrapper checks passed"
