#![allow(unused_imports)]

#[allow(unused_imports)]
use extendr_api::prelude::*;

struct Foo;

#[extendr]
fn get_foo() -> Foo {
    Foo
}

#[extendr]
fn use_foo(_foo: Foo) {}
