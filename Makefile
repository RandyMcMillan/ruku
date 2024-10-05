# Default target
.PHONY: all
all: build

# Build the project
.PHONY: build
build:
	cargo build

# Clean the project
.PHONY: clean
clean:
	cargo clean

# Run tests
.PHONY: test
test:
	cargo test

# Format the code
.PHONY: format
format:
	cargo fmt

# Lint the code
.PHONY: lint
lint:
	cargo clippy -- -D warnings

