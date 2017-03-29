# libOSTree bindings for Rust

This crate implements Rust bindings for OSTree using the GObject introspection XML file.
It uses [gir](https://github.com/gtk-rs/gir) to autogenerate the Rust API code.


## Using ostree

Insert examples here


## Rebuilding ostree

 * Check out the repository
 * Make sure you have the OSTree-1.0.gir file installed on your system. The Makefile
   looks for it in the path output from `pkg-config --variable=girdir gobject-introspection-1.0`
 * `cd ostree-sys`
 * `make gir` will regenerate the src/lib.rs file
 * `cd .. && make gir` will regenerate the src/auto/ files
 * `cargo build --release` will build the crate.

