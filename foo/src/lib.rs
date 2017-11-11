#[macro_export]
macro_rules! a {
    ($a:ident) => (
        fn _bar() {
            mem::drop(3);
        }
    )
}

