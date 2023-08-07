

run:
	@echo "Running..."
	@cargo run -- --program-file test_programs/valid/if_blocks.lrlang

dist:
	@cargo build --release

build:
	@echo "Building..."
	@cargo build

test:
	@echo "Running tests..."
	@cargo test