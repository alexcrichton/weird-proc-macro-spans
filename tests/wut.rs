#![feature(proc_macro)]
extern crate wut;

use wut::gobject_gen;

gobject_gen! {
}

#[test]
fn foo() {
    let _a = A;
}
