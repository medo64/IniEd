ifeq (, $(shell which cargo))
    $(error "No 'cargo' in path, consider installing rust")
endif

ifeq ($(PREFIX),)
    PREFIX := /usr/local/
endif


SOURCE_LIST := Cargo.lock Cargo.toml LICENSE.md Makefile README.md src/


.PHONY: all clean distclean install uninstall dist release debug


all: release


clean:
	-@$(RM) -r bin/
	-@$(RM) -r build/

distclean: clean
	-@$(RM) -r target/


install: bin/inied
	@sudo install -d $(DESTDIR)/$(PREFIX)/bin/
	@sudo install bin/inied $(DESTDIR)/$(PREFIX)/bin/

uninstall: $(DESTDIR)/$(PREFIX)/bin/inied
	@sudo $(RM) $(DESTDIR)/$(PREFIX)/bin/inied

dist: release
	@$(RM) -r build/dist/
	@$(eval DIST_NAME = $(shell bin/inied -s package -k name -p Cargo.toml))
	@$(eval DIST_VERSION = $(shell bin/inied -s package -k version -p Cargo.toml))
	@mkdir -p build/dist/$(DIST_NAME)-$(DIST_VERSION)/
	@cp -r $(SOURCE_LIST) build/dist/$(DIST_NAME)-$(DIST_VERSION)/
	@tar -cz -C build/dist/ -f build/dist/$(DIST_NAME)-$(DIST_VERSION).tar.gz $(DIST_NAME)-$(DIST_VERSION)/
	@mkdir -p dist/
	@mv build/dist/$(DIST_NAME)-$(DIST_VERSION).tar.gz dist/


release: src/main.rs
	@mkdir -p bin/
	@cargo build --release --target-dir build/
	@cp build/release/inied bin/inied

debug: src/main.rs
	@cargo build --target-dir build
	@mkdir -p bin/
	@cp build/debug/inied bin/inied
