BINARY_NAME = nvramhof
INSTALL_DIR=$(HOME)/.local/bin

.PHONY: all build run clean install

all: build

build:
	cargo build --release

run:
	cargo run --release

clean:
	cargo clean

install: build
	mkdir -p $(INSTALL_DIR)
	cp target/release/$(BINARY_NAME) $(INSTALL_DIR)/$(BINARY_NAME)
	chmod +x $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "Installed - $(INSTALL_DIR)/$(BINARY_NAME)"
