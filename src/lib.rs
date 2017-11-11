#![feature(proc_macro)]

extern crate proc_macro;

use proc_macro::*;

fn tt(kind: TokenNode) -> TokenTree {
    TokenTree {
        span: Span::default(),
        kind,
    }
}

fn call_site_all(t: TokenStream) -> TokenStream {
    t.into_iter().map(|mut tt| {
        tt.span = Span::call_site();
        tt.kind = match tt.kind {
            TokenNode::Group(d, ts) => TokenNode::Group(d, call_site_all(ts)),
            node => node,
        };
        tt
    }).collect()
}

fn term(s: &str) -> TokenTree {
    tt(TokenNode::Term(Term::intern(s)))
}

fn op(c: char, s: Spacing) -> TokenTree {
    tt(TokenNode::Op(c, s))
}

fn braces(v: Vec<TokenTree>) -> TokenTree {
    tt(TokenNode::Group(Delimiter::Brace, v.into_iter().collect()))
}

fn parse(s: &str) -> TokenTree {
    s.parse::<TokenStream>().unwrap().into_iter().next().unwrap()
}

// ============================================================================
//
// Spans of tokens like `super` are very sensitive. This example shows that if
// you have a "bug" (is it a bug?) like [1] it can get super weird.
//
// [1]: https://github.com/dtolnay/quote/pull/51

#[proc_macro]
pub fn failure1(_input: TokenStream) -> TokenStream {
    let v = vec![
        term("mod"), term("foo"),
        braces(vec! [
            term("use"),
            parse("super"), // note that this is `parse` ...
            op(':', Spacing::Joint),
            op(':', Spacing::Alone),
            op('*', Spacing::Alone),
            op(';', Spacing::Alone),
        ]),
    ];
    return v.into_iter().collect()
}

#[proc_macro]
pub fn success1(_input: TokenStream) -> TokenStream {
    let v = vec![
        term("mod"), term("foo"),
        braces(vec! [
            term("use"),
            term("super"), // ... and this is `term`
            op(':', Spacing::Joint),
            op(':', Spacing::Alone),
            op('*', Spacing::Alone),
            op(';', Spacing::Alone),
        ]),
    ];
    return v.into_iter().collect()
}

// ============================================================================
//
// If you use the default span you can't import your own structs!

#[proc_macro]
pub fn failure2(_input: TokenStream) -> TokenStream {
    // struct A;
    // mod foo {
    //      use super::A;
    // }
    let v = vec![
        term("struct"), term("A"), op(';', Spacing::Alone),
        term("mod"), term("foo"),
        braces(vec! [
               term("use"),
               term("super"),
               op(':', Spacing::Joint),
               op(':', Spacing::Alone),
               term("A"),
               op(';', Spacing::Alone),
        ]),
    ];
    return v.into_iter().collect()
}

#[proc_macro]
pub fn success2(input: TokenStream) -> TokenStream {
    call_site_all(failure2(input))
}

// ============================================================================
// If you use the default span then outside code can't import from the generated
// code, see the test case for more info.

#[proc_macro]
pub fn failure3(_input: TokenStream) -> TokenStream {
    let v = vec![
        term("pub"), term("struct"), term("A"), op(';', Spacing::Alone),
    ];
    return v.into_iter().collect()
}

#[proc_macro]
pub fn success3(input: TokenStream) -> TokenStream {
    call_site_all(failure3(input))
}

// ============================================================================
// If you use the default span you can't import from `std`.

#[proc_macro]
pub fn failure4(_input: TokenStream) -> TokenStream {
    let v = vec![
        term("use"),
        term("std"),
        op(':', Spacing::Joint),
        op(':', Spacing::Alone),
        term("mem"),
        op(';', Spacing::Alone),
    ];
    return v.into_iter().collect()
}

#[proc_macro]
pub fn success4(input: TokenStream) -> TokenStream {
    call_site_all(failure4(input))
}

// ============================================================================
// If you use the default span you can't import from your own modules

#[proc_macro]
pub fn failure5(_input: TokenStream) -> TokenStream {
    // use foo::*;
    // mod foo {}
    let v = vec![
        term("use"),
        term("foo"),
        op(':', Spacing::Joint),
        op(':', Spacing::Alone),
        op('*', Spacing::Alone),
        op(';', Spacing::Alone),

        term("mod"),
        term("foo"),
        braces(vec![]),
    ];
    return v.into_iter().collect()
}

#[proc_macro]
pub fn success5(input: TokenStream) -> TokenStream {
    call_site_all(failure5(input))
}

// ============================================================================
// Here we're using a "malformed" macro named `a` from our local crate `foo`.
// This malformed macro requires the surrounding code to import `std::mem`. If
// we do that, however, it doesn't work!

#[proc_macro]
pub fn failure6(_input: TokenStream) -> TokenStream {
    // mod another {
    //      extern crate std;
    //      use self::std::mem;
    //      a! {}
    // }
    let v = vec![
        term("mod"), term("another"), braces(vec![
            term("extern"), term("crate"), term("std"), op(';', Spacing::Alone),
            term("use"), term("self"),
                op(':', Spacing::Joint),
                op(':', Spacing::Alone),
                term("std"),
                op(':', Spacing::Joint),
                op(':', Spacing::Alone),
                term("mem"),
                op(';', Spacing::Alone),

            term("a"),
            op('!', Spacing::Alone),
            braces(vec! [
                term("_A"),
            ]),
        ]),
    ];
    return v.into_iter().collect()
}

// using the `call_site` span also doesn't work...
#[proc_macro]
pub fn failure6_2(input: TokenStream) -> TokenStream {
    call_site_all(failure6(input))
}

// I don't think this can succeed unless the original macro is fixed...
