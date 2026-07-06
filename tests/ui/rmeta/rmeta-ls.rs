//@ no-prefer-dynamic
//@ compile-flags: --emit=metadata
//@ aux-build:rmeta-meta.rs
//@ revisions: rpass1 rpass2
//@ [rpass2]compile-flags: -Zls=all
//@ check-pass

extern crate rmeta_meta;
use rmeta_meta::Foo;

fn main() {
    let _ = Foo { field: 42 };
}

pub fn foo() {}
fn bar() {}
fn baz() {
    bar()
}
