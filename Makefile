# Phony targets
.PHONY: lint fmt test test-cli check all

# --- Variables ---
CARGO := cargo
CLIPPY_FLAGS := --all-targets -- -D warnings
FMT_FLAGS := --all -- --check
TEST_SCRIPT := bash scripts/test_cli.sh

# --- Targets ---

# Default target: run full CI checks
all: check

# Run clippy with strict warnings (fail on any warning)
lint:
	@echo "=== Running clippy (strict mode) ==="
	$(CARGO) clippy $(CLIPPY_FLAGS)

# Check Rust formatting without changing files
fmt:
	@echo "=== Checking code formatting ==="
	$(CARGO) fmt $(FMT_FLAGS)

# Run CLI integration tests
test-cli:
	@echo "=== Running CLI tests ==="
	$(TEST_SCRIPT)

# Alias for test-cli (backward compatibility)
test: test-cli

# Full CI pipeline: lint -> fmt -> test
check: lint fmt test
	@echo "=== All checks passed! ==="
