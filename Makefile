

run:
	@echo "Running..."
	@cargo run -- --program-file test_programs/valid/if_blocks.lrlang

build:
	@echo "Building..."
	@cargo build

test:
	@echo "Running tests..."
	@cargo test