PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
MANDIR ?= $(PREFIX)/share/man

.PHONY: all build test install uninstall clean

all: build

build:
	cargo build --release

test:
	cargo test
	cargo test --test integration

install: build
	install -d $(DESTDIR)$(BINDIR)
	install -m 755 target/release/1seed $(DESTDIR)$(BINDIR)/1seed

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/1seed

clean:
	cargo clean

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

check: fmt lint test
