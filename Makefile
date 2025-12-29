PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
MANDIR ?= $(PREFIX)/share/man

.PHONY: all build test install uninstall clean release

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

release:
	@if [ -z "$(VERSION)" ]; then echo "Usage: make release VERSION=0.2.0"; exit 1; fi
	@echo "Releasing version $(VERSION)..."
	sed -i.bak 's/^version = ".*"/version = "$(VERSION)"/' Cargo.toml && rm Cargo.toml.bak
	git add Cargo.toml
	git commit -m "Bump version to v$(VERSION)"
	git tag -a v$(VERSION) -m "Release v$(VERSION)"
	git push origin master
	git push origin v$(VERSION)
	@echo "Release v$(VERSION) created and pushed!"
	@echo "GitHub Actions will build binaries at: https://github.com/oeo/1seed/actions"
