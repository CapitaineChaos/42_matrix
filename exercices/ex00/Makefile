.ONESHELL:

TARGET = ex00

all:
	. ../../scripts/install_cargo_goinfre.sh
	. ../../scripts/use_cargo.sh
	cargo build --release --bin $(TARGET)
	cargo run --bin $(TARGET)

build:
	. ../../scripts/install_cargo_goinfre.sh
	. ../../scripts/use_cargo.sh
	cargo build --release --bin $(TARGET)

clean:
	. ../../scripts/use_cargo.sh
	cargo clean

fclean: clean
	rm -rf $(TARGET)

remove-cargo:
	rm -rf /home/$USER/goinfre/.cargo
	rm -rf /home/$USER/goinfre/.rustup

.PHONY: all build clean fclean
