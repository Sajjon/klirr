set quiet := true
set shell := ["bash", "-cu"]

# Generate invoice and open the produced PDF
service: (_make-and-open "cargo run --bin klirr invoice")

# Show CLI help for the invoice binary
help:
  cargo run --bin invoice -- --help

# Generate expenses invoice and open the produced PDF
expense: (_make-and-open "cargo run --bin klirr invoice expenses")

# Usage: `just ooo 5`
ooo days_off: (_make-and-open "cargo run --bin klirr invoice ooo {{days_off}}")

_make-and-open cmd:
  #!/usr/bin/env bash
  set -euo pipefail
  tmp_output="$(mktemp)"
  cleanup() { rm -f "$tmp_output"; }
  trap cleanup EXIT

  if TMP_FILE_FOR_PATH_TO_PDF="$tmp_output" {{cmd}}; then
    output_path="$(cat "$tmp_output")"
    open "$output_path"
  else
    exit_code=$?
    echo "Error: command failed with exit code $exit_code"
    exit "$exit_code"
  fi
