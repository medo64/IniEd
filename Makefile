ifeq (, $(shell which cargo))
    $(error "No 'cargo' in path, consider installing rust")
endif

ifeq ($(PREFIX),)
    PREFIX := /usr/local/
endif


.PHONY: all clean distclean install uninstall release debug


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


release: src/main.rs
	@mkdir -p bin/
	@cargo build --release --target-dir build/
	@cp build/release/inied bin/inied

debug: src/main.rs
	@cargo build --target-dir build
	@mkdir -p bin/
	@cp build/debug/inied bin/inied
