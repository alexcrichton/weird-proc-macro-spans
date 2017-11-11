pub trait A {
    fn foo(&self) {
    }
}

#[macro_export]
macro_rules! a {
    ($a:ident) => (
        mod fancy {
            struct $a;
            impl $crate::A for $a {
                fn foo(&self) {
                    mem::drop(self);
                }
            }
        }
    )
}

