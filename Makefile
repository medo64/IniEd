ifeq ($(PREFIX),)
    PREFIX := /usr/local/
endif

CARGO_PLATFORM := $(shell getconf LONG_BIT | sed "s/32/i686-unknown-linux-gnu/" | sed "s/64/x86_64-unknown-linux-gnu/")
DEB_BUILD_ARCH := $(shell getconf LONG_BIT | sed "s/32/i386/" | sed "s/64/amd64/")


SOURCE_LIST := Cargo.lock Cargo.toml LICENSE.md Makefile README.md src/ docs/


.PHONY: all clean distclean install uninstall dist release debug package


all: release


clean:
	-@$(RM) -r bin/
	-@$(RM) -r build/

distclean: clean
	-@$(RM) -r dist/
	-@$(RM) -r target/


install: bin/inied
	@sudo install -d $(DESTDIR)/$(PREFIX)/bin/
	@sudo install bin/inied $(DESTDIR)/$(PREFIX)/bin/
	@$(RM) -r build/man/
	@mkdir -p build/man/
	@gzip -c docs/man/inied.1 > build/man/inied.1.gz
	@sudo install -m 644 build/man/inied.1.gz /usr/share/man/man1/
	@sudo mandb -q

uninstall: $(DESTDIR)/$(PREFIX)/bin/inied
	@sudo $(RM) $(DESTDIR)/$(PREFIX)/bin/inied
	@sudo $(RM) /usr/share/man/man1/inied.1.gz
	@sudo mandb -q

dist: release
	@$(eval DIST_NAME = $(shell bin/inied -s package -k name -p Cargo.toml))
	@$(eval DIST_VERSION = $(shell bin/inied -s package -k version -p Cargo.toml))
	@$(RM) -r build/dist/
	@mkdir -p build/dist/$(DIST_NAME)-$(DIST_VERSION)/
	@cp -r $(SOURCE_LIST) build/dist/$(DIST_NAME)-$(DIST_VERSION)/
	@tar -cz -C build/dist/  --owner=0 --group=0 -f build/dist/$(DIST_NAME)-$(DIST_VERSION).tar.gz $(DIST_NAME)-$(DIST_VERSION)/
	@mkdir -p dist/
	@mv build/dist/$(DIST_NAME)-$(DIST_VERSION).tar.gz dist/
	@echo Output at dist/$(DIST_NAME)-$(DIST_VERSION).tar.gz


release: src/main.rs
	@command -v cargo >/dev/null 2>&1 || { echo >&2 "No 'cargo' in path, consider installing rust!"; exit 1; }
	@echo "Building for $(CARGO_PLATFORM)"
	@mkdir -p bin/
	@cargo build --release --quiet --target $(CARGO_PLATFORM) --target-dir build/
	@cp build/$(CARGO_PLATFORM)/release/inied bin/inied

debug: src/main.rs
	@command -v cargo >/dev/null 2>&1 || { echo >&2 "No 'cargo' in path, consider installing rust!"; exit 1; }
	@echo "Building for $(CARGO_PLATFORM)"
	@cargo build --target $(CARGO_PLATFORM) --target-dir build
	@mkdir -p bin/
	@cp build/$(CARGO_PLATFORM)/debug/inied bin/inied


package: dist
	@command -v dpkg-deb >/dev/null 2>&1 || { echo >&2 "Package 'dpkg-deb' not installed!"; exit 1; }
	@echo "Packaging for $(DEB_BUILD_ARCH)"
	@$(eval DIST_NAME = $(shell bin/inied -s package -k name -p Cargo.toml))
	@$(eval DIST_VERSION = $(shell bin/inied -s package -k version -p Cargo.toml))
	@$(eval PACKAGE_NAME = inied_$(DIST_VERSION)_$(DEB_BUILD_ARCH))
	@$(eval PACKAGE_DIR = /tmp/$(PACKAGE_NAME)/)
	-@$(RM) -r $(PACKAGE_DIR)/
	@mkdir $(PACKAGE_DIR)/
	@cp -r package/deb/DEBIAN $(PACKAGE_DIR)/
	@sed -i "s/MAJOR.MINOR/$(DIST_VERSION)/" $(PACKAGE_DIR)/DEBIAN/control
	@sed -i "s/ARCHITECTURE/$(DEB_BUILD_ARCH)/" $(PACKAGE_DIR)/DEBIAN/control
	@mkdir -p $(PACKAGE_DIR)/usr/share/man/man1/
	@gzip -c --best docs/man/inied.1 > $(PACKAGE_DIR)/usr/share/man/man1/inied.1.gz
	@find $(PACKAGE_DIR)/ -type d -exec chmod 755 {} +
	@find $(PACKAGE_DIR)/ -type f -exec chmod 644 {} +
	@chmod 755 $(PACKAGE_DIR)/DEBIAN/p*inst $(PACKAGE_DIR)/DEBIAN/p*rm
	@install -d $(PACKAGE_DIR)/usr/bin/
	@install bin/inied $(PACKAGE_DIR)/usr/bin/
	@strip $(PACKAGE_DIR)/usr/bin/inied
	@fakeroot dpkg-deb --build $(PACKAGE_DIR)/ > /dev/null
	@cp /tmp/$(PACKAGE_NAME).deb dist/
	@$(RM) -r $(PACKAGE_DIR)/
	@echo Output at dist/$(PACKAGE_NAME).deb
