

run:
	@echo "Running..."
	@cargo run -- test_programs/valid/if_blocks.lrlang

dist:
	@cargo install --path .
	@cargo build --release

build:
	@echo "Building..."
	@cargo build

test:
	@echo "Running tests..."
	@cargo test