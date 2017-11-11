#![feature(proc_macro)]
extern crate wut;

use wut::failure3;

failure3! {
}

#[test]
fn foo() {
    let _a = A;
}
