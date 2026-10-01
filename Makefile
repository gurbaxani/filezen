PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin

.PHONY: all build release install uninstall test clean

all: build

build:
	cargo build

release:
	cargo build --release

install: release
	install -d $(DESTDIR)$(BINDIR)
	install -m 755 target/release/filezen $(DESTDIR)$(BINDIR)/filezen

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/filezen

test:
	cargo test

clean:
	cargo clean
