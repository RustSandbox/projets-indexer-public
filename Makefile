# Makefile for Ollama Rust Client

# Variables
CARGO := cargo
RELEASE_FLAGS := --release

.PHONY: build install help index search stats tags clean test doc lint format

# Default target
help:
	@echo "Projects Indexer - Available commands:"
	@echo ""
	@echo "Usage:"
	@echo "  make <command>"
	@echo ""
	@echo "Commands:"
	@echo "  build              Build the project in release mode"
	@echo "  install            Install the binary globally"
	@echo "  clean              Clean build artifacts"
	@echo ""
	@echo "Index Commands:"
	@echo "  index              Index projects with default settings"
	@echo "  index-custom       Index projects with custom directory and output"
	@echo "  index-deep         Index projects with deeper traversal"
	@echo "  index-no-ollama    Index projects without Ollama tag generation"
	@echo "  index-verbose      Index projects with verbose output"
	@echo ""
	@echo "Search Commands:"
	@echo "  search-all         Search for projects containing 'rust'"
	@echo "  search-tags        Search only in project tags"
	@echo "  search-category    Search only in project categories"
	@echo ""
	@echo "Stats Commands:"
	@echo "  stats              Show basic project statistics"
	@echo "  stats-detailed     Show detailed project statistics"
	@echo ""
	@echo "Tag Commands:"
	@echo "  generate-tags      Generate tags for a specific project"
	@echo ""
	@echo "Development Commands:"
	@echo "  dev-build          Build in debug mode"
	@echo "  dev-run            Run with verbose output in debug mode"
	@echo "  test               Run tests"
	@echo "  doc                Generate and open documentation"
	@echo "  lint               Run clippy lints"
	@echo "  format             Check code formatting"

# Build commands
build:
	$(CARGO) build $(RELEASE_FLAGS)

install: build
	$(CARGO) install --path .

clean:
	$(CARGO) clean

# Index commands
index:
	$(CARGO) run $(RELEASE_FLAGS) -- index

index-custom:
	$(CARGO) run $(RELEASE_FLAGS) -- index -d ~/projects/custom -o custom-index.json

index-deep:
	$(CARGO) run $(RELEASE_FLAGS) -- index -d ~/projects -x 5 -m 2

index-no-ollama:
	$(CARGO) run $(RELEASE_FLAGS) -- index --no-ollama

index-verbose:
	$(CARGO) run $(RELEASE_FLAGS) -- index -v

# Search commands
search-all:
	$(CARGO) run $(RELEASE_FLAGS) -- search "rust"

search-tags:
	$(CARGO) run $(RELEASE_FLAGS) -- search --tags-only "web"

search-category:
	$(CARGO) run $(RELEASE_FLAGS) -- search --category-only "machine-learning"

# Stats commands
stats:
	$(CARGO) run $(RELEASE_FLAGS) -- stats

stats-detailed:
	$(CARGO) run $(RELEASE_FLAGS) -- stats --detailed

# Tag commands
generate-tags:
	$(CARGO) run $(RELEASE_FLAGS) -- generate-tags -p ~/projects/my-project -o tags.json

# Development helpers
dev-build:
	$(CARGO) build

dev-run:
	$(CARGO) run -- index -v

test:
	$(CARGO) test

doc:
	$(CARGO) doc --no-deps --document-private-items --open

lint:
	$(CARGO) clippy -- -D warnings

format:
	$(CARGO) fmt --all -- --check 