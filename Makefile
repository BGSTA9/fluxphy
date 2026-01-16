PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin

.PHONY: all build install clean uninstall test check fmt lint release

all: build

build:
	cargo build --release

install: build
	install -Dm755 target/release/fluxphy $(DESTDIR)$(BINDIR)/fluxphy

clean:
	cargo clean

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/fluxphy

test:
	cargo test

check:
	cargo check

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

release: clean
	cargo build --release
	@echo "Release binary at: target/release/fluxphy"
