all: build

clean:
	@echo "Cleaning up ..."
	@cargo clean

build: problem*.rs
	@echo "Creating Cargo.toml"
	@python build_cargo.py
	@echo "Building ..."
	@cargo build
	@echo "Done."
