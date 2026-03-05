PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
MANDIR ?= $(PREFIX)/share/man
COMPLETIONSDIR ?= $(PREFIX)/share
LICENSEDIR ?= $(PREFIX)/share/licenses/1seed

.PHONY: all build test install uninstall clean generate fmt lint check bump release

all: build

build:
	cargo build --release

test:
	cargo test
	cargo test --test integration

generate: build
	cargo build --release --features generate --bin 1seed-generate
	./target/release/1seed-generate .

install: build generate
	install -d $(DESTDIR)$(BINDIR)
	install -m 755 target/release/1seed $(DESTDIR)$(BINDIR)/1seed
	install -d $(DESTDIR)$(MANDIR)/man1
	install -m 644 man/1seed.1 $(DESTDIR)$(MANDIR)/man1/1seed.1
	install -d $(DESTDIR)$(COMPLETIONSDIR)/bash-completion/completions
	install -m 644 completions/1seed.bash $(DESTDIR)$(COMPLETIONSDIR)/bash-completion/completions/1seed
	install -d $(DESTDIR)$(COMPLETIONSDIR)/zsh/vendor-completions
	install -m 644 completions/1seed.zsh $(DESTDIR)$(COMPLETIONSDIR)/zsh/vendor-completions/_1seed
	install -d $(DESTDIR)$(COMPLETIONSDIR)/fish/vendor_completions.d
	install -m 644 completions/1seed.fish $(DESTDIR)$(COMPLETIONSDIR)/fish/vendor_completions.d/1seed.fish
	install -d $(DESTDIR)$(LICENSEDIR)
	install -m 644 LICENSE $(DESTDIR)$(LICENSEDIR)/LICENSE

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/1seed
	rm -f $(DESTDIR)$(MANDIR)/man1/1seed.1
	rm -f $(DESTDIR)$(COMPLETIONSDIR)/bash-completion/completions/1seed
	rm -f $(DESTDIR)$(COMPLETIONSDIR)/zsh/vendor-completions/_1seed
	rm -f $(DESTDIR)$(COMPLETIONSDIR)/fish/vendor_completions.d/1seed.fish
	rm -rf $(DESTDIR)$(LICENSEDIR)

clean:
	cargo clean

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

check: fmt lint test

bump:
	@echo "Auto-incrementing version..."
	@current=$$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'); \
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
	echo "Current: $$current -> New: $$new_version"; \
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
