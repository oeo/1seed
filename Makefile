PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
MANDIR ?= $(PREFIX)/share/man

.PHONY: all build test install uninstall clean release bump

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

bump:
	@echo "Auto-incrementing version..."
	@current=$$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'); \
	major=$$(echo $$current | cut -d. -f1); \
	minor=$$(echo $$current | cut -d. -f2); \
	patch=$$(echo $$current | cut -d. -f3); \
	if [ "$(TYPE)" = "major" ]; then \
		major=$$((major + 1)); minor=0; patch=0; \
	elif [ "$(TYPE)" = "minor" ]; then \
		minor=$$((minor + 1)); patch=0; \
	else \
		patch=$$((patch + 1)); \
	fi; \
	new_version="$$major.$$minor.$$patch"; \
	echo "Current: $$current → New: $$new_version"; \
	$(MAKE) release VERSION=$$new_version

release:
	@if [ -z "$(VERSION)" ]; then echo "Usage: make release VERSION=0.2.0"; exit 1; fi
	@echo "Releasing version $(VERSION)..."
	sed -i.bak 's/^version = ".*"/version = "$(VERSION)"/' Cargo.toml && rm Cargo.toml.bak
	git add Cargo.toml
	git commit -m "Bump version to v$(VERSION)"
	git tag -a v$(VERSION) -m "Release v$(VERSION)"
	git push origin master
	git push origin v$(VERSION)
	@echo ""
	@echo "Release v$(VERSION) created and pushed!"
	@echo "GitHub Actions will build binaries at: https://github.com/oeo/1seed/actions"
	@echo ""
	@echo "To publish to crates.io, run: cargo publish"
