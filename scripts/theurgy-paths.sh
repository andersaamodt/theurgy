#!/bin/sh

theurgy_state_root() {
  printf '%s\n' "${THEURGY_STATE_DIR:-${XDG_STATE_HOME:-$HOME/.local/state}/theurgy}"
}

theurgy_cargo_target_dir() {
  printf '%s\n' "${THEURGY_CARGO_TARGET_DIR:-$(theurgy_state_root)/cargo-target}"
}

theurgy_export_cargo_env() {
  export CARGO_TARGET_DIR
  CARGO_TARGET_DIR=$(theurgy_cargo_target_dir)
}
