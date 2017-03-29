GIR = gir/target/bin/gir
GIR_SRC = gir/Cargo.toml gir/Cargo.lock gir/build.rs $(shell find gir/src -name '*.rs')
#GIR_FILES = gir-files/Gtk-3.0.gir
GIR_DIR = $(shell pkg-config --variable=girdir gobject-introspection-1.0)
GIR_FILES = $(GIR_DIR)/OSTree-1.0.gir

# Run `gir` generating the bindings
gir : src/auto/mod.rs

src/auto/mod.rs : Gir.toml $(GIR) $(GIR_FILES)
	rm -f gir-files
	ln -s $(GIR_DIR) gir-files
	$(GIR) -c Gir.toml

$(GIR) : $(GIR_SRC)
	rm -f gir/target/bin/gir
	cargo install --path gir --root gir/target
	rm -f gir/target/.crates.toml
