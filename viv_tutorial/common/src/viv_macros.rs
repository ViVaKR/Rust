// use macro_rules! <name of macro> {<Body>}

#[macro_export]
macro_rules! add {
    // macth like arm for macro
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

#[macro_export]
macro_rules! add_as {
    ($a:expr, $b:expr, $typ:ty) => {
        $a as $typ + $b as $typ
    };
}

#[macro_export]
macro_rules! adds {
    ($($a:expr), *) => {
        {
            0
            $(+$a)*
        }
    };
}

#[macro_export]
macro_rules! addm {
    // first arm
    ($a:expr) => {
        $a
    };

    // second arm
    ($a:expr, $b:expr) => {
        {$a + $b}
    };

    // add the number and the result of remaining arguments
    ($a:expr, $($b:tt)*) => {
        $a + addm!($($b)*)
    }
}

/*
--> arguments
- item -- an item, like a function, struct, module, etc.
- block -- a block (i.e. a block of statements and/or an expression, surrounded by braces)
- stmt -- a statement
- pat -- a pattern
- expr -- an expression
- ty -- a type
- ident -- an identifier
- path -- a path (e.g., foo, ::std::emm::replace, transmut::<_, int>, ...)
- meta -- ameta item; the thins that go inside #[...] and #![...] attributes
- tt -- a single token tree
- vis -- a possibly empty Visibility qualifier
*/
