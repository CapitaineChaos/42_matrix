.ONESHELL:

# 	compiler projet vide et tester sous valgrind
all:
	. ../scripts/cargo.sh
	cargo build
	@echo "Build complete"
	@valgrind ./target/debug/main