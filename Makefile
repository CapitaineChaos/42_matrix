.ONESHELL:


EX := $(word 2,$(MAKECMDGOALS))

all:
	@echo "Matrix 42"


clean:
	. ./scripts/cargo.sh
	cargo clean

fclean: clean
	rm -rf target

%:
	@true

remove-cargo:
	rm -rf /home/$USER/goinfre/.cargo
	rm -rf /home/$USER/goinfre/.rustup

ex:
	. ./scripts/cargo.sh
	cargo run --bin ex${EX}


.PHONY: all clean fclean remove-cargo ex
