.ONESHELL:


EX := $(word 2,$(MAKECMDGOALS))

all:
	@echo "Matrix 42"
	. ./scripts/cargo.sh

clippy:
	. ./scripts/cargo.sh
	cargo clippy

check:
	. ./scripts/cargo.sh
	cargo check

clean:
	. ./scripts/cargo.sh
	cargo clean

fclean: clean
	rm -rf target

%:
	@true

remove-cargo:
	rm -rf /tmp/.cargo
	rm -rf /tmp/.rustup
ex:
	. ./scripts/cargo.sh
	cargo run --release --example ex${EX}

bonus_ex:
	. ./scripts/cargo.sh
	cargo run --release --example ex${EX}_bonus


.PHONY: all clean fclean remove-cargo ex bonus_ex clippy
