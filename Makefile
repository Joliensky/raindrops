# Name deines Programms (muss mit dem Namen in der Cargo.toml übereinstimmen)
BINARY_NAME=raindrops

all:
	cargo build --release

install:
	install -m 755 target/release/$(BINARY_NAME) /usr/local/bin/$(BINARY_NAME)

uninstall:
	rm -f /usr/local/bin/$(BINARY_NAME)