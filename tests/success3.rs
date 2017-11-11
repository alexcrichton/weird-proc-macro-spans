#![feature(proc_macro)]
extern crate wut;

use wut::success3;

success3! {
}

#[test]
fn foo() {
    let _a = A;
}
