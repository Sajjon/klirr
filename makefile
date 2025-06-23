.PHONY: help, _make_and_open, ooo, service, expense

service:
	@CMD="cargo run --bin klirr invoice" $(MAKE) _make_and_open

help:
	cargo run --bin invoice -- --help

expense:
	@CMD="cargo run --bin klirr invoice expenses" $(MAKE) _make_and_open
	
_make_and_open:
	@TMP_OUTPUT=$$(mktemp); \
	TMP_FILE_FOR_PATH_TO_PDF=$$TMP_OUTPUT CMD="TMP_FILE_FOR_PATH_TO_PDF=$$TMP_OUTPUT $$CMD" $(MAKE) _run_and_open; \
	rm -f $$TMP_OUTPUT

_run_and_open:
	@eval "$$CMD"; \
	EXIT_CODE=$$?; \
	if [ $$EXIT_CODE -eq 0 ]; then \
		OUTPUT_PATH=$$(cat $$TMP_FILE_FOR_PATH_TO_PDF); \
		open "$$OUTPUT_PATH"; \
	else \
		echo "Error: command failed with exit code $$EXIT_CODE"; \
		exit $$EXIT_CODE; \
	fi

# Usage: `make ooo DAYS_OFF=5`
ooo:
	@if [ -z "$(DAYS_OFF)" ]; then \
		echo "Error: DAYS_OFF is required"; \
		exit 1; \
	fi; \
	CMD="cargo run --bin klirr invoice ooo $(DAYS_OFF)" $(MAKE) _make_and_open
