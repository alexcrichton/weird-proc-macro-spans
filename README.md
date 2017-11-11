# Spans in procedural macros

The motivation for creating this README and repo was originally from the
[gnome-class] project. This project is defining a procedural macro with a small
DSL to conveniently declare glib-compatible types and interface to them from
both Rust and C. The crate is currently (as of this writing) undergoing a
rewrite to transition to a "new" procedural macro which preserves span
information to ensure that type errors and such in user-written code are
legible.

So what I did was enable the `unstable` feature of the `proc-macro2` crate
(which forces it to implement itself with the upstream `proc_macro` crate) and
then oh boy down the rabbit hole I went...

## Organization

This project has a few pieces:

* `src/lib.rs` - this is all the procedural macros for all the tests. There's
  some more comments about the file internally.
* `tests/{failure,success}N.rs` - these are a bunch of files showing off whether
  compilation fails or succeeds. For example `failure1.rs` is an example of a
  procedural macro invocation that fails to compile, while `success1.rs` is a
  small tweak on behalf of the procedural macro author's side of things which
  gets the procedural macro to succeed to compile.
* `foo` - a helper crate for some of the test cases.

For example here's what happened with the `failure2` test:

```
$ cargo +nightly test --test failure2
   Compiling wut v0.1.0 (file:///Users/acrichton/code/weird-proc-macro-spans)
error[E0432]: unresolved import `super`
 --> tests/failure2.rs:8:1
  |
8 | / failure2! {
9 | | }
  | |_^ no `A` in the root

error: aborting due to previous error

error: Could not compile `wut`.
```

This was an example, if we look at the procedural macro definition, of how a
generated module can't import contents defined outside. We can see the fix,
however:

```
$ cargo +nightly test --test success2
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/success2-c6ae2bc09f230207

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

One possible fix in this case, looking at the definition of `success2`, was to
switch all spans to `call_site`.

[gnome-class]: https://github.com/federicomenaquintero/gnome-class
