#!/bin/sh

set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd -P)
tmp_dir=$(mktemp -d "${TMPDIR:-/tmp}/theurgy-firewall-darwin.XXXXXX")
trap 'rm -rf "$tmp_dir"' EXIT HUP INT TERM

help_output=$(sh "$ROOT_DIR/spells/firewall-darwin" --help)
printf '%s' "$help_output" | grep -F "Usage: firewall-darwin" >/dev/null

if [ "$(uname -s 2>/dev/null || printf unknown)" != "Darwin" ]; then
  printf '%s\n' "firewall-darwin tests skipped on non-Darwin host"
  exit 0
fi

listener_log="$tmp_dir/listener.log"
nc -l 127.0.0.1 18443 >"$listener_log" 2>/dev/null &
listener_pid=$!
sleep 1

sh "$ROOT_DIR/spells/firewall-darwin" --allow-localhost -- nc -z 127.0.0.1 18443

if sh "$ROOT_DIR/spells/firewall-darwin" -- nc -z 127.0.0.1 18443 >/dev/null 2>&1; then
  printf '%s\n' "firewall-darwin allowed localhost traffic without --allow-localhost" >&2
  kill "$listener_pid" >/dev/null 2>&1 || true
  wait "$listener_pid" 2>/dev/null || true
  exit 1
fi

kill "$listener_pid" >/dev/null 2>&1 || true
wait "$listener_pid" 2>/dev/null || true

printf '%s\n' "firewall-darwin checks passed"
