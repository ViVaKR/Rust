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

#[macro_export]
macro_rules! ok_or_return {
    // internal rule.
    (@error $a:ident, $($b:tt)*) => {
        {
            match $a($($b)*) {
                Ok(value) => value,
                Err(err) => {
                    return Err(err);
                }
            }
        }
    };

    // public rule can be called by the user
    ($a:ident($($b:tt)*)) => {
        ok_or_return!(@error $a, $($b)*)
    };
}

#[macro_export]
macro_rules! make_public {
    // use vis type for visibility keyword
    // and
    // ident for struct name
    (
        // meta data about struct
        $(#[$meta:meta])*
        $vis:vis struct $struct_name:ident
        {
            // vis for fiedl visibility,
            // ident for fiedl name,
            // ty for field data type,

            $(
                // meta data about field
                $(#[$field_meta:meta])*
                $field_vis:vis $field_name:ident : $ field_type:ty
            ), *$(,)+
            // $field_vis:vis $field_anme:ident : $fiedl_type:ty
        }
    ) => {

        {
            $(#[$meta])*
            pub struct $struct_name {
                $(
                    $(#[$field_meta:meta])*
                    pub $field_name : $field_type,
                )*
            }
        }
    };
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

--> pub struct MyStruct {}
- pub : Visibility keyword(vis token type)
- struct : struct keyword
- MyStruct : struct name
- {} : struct body

--> pub fieldName:type,
- pub : Visibility key word
- fieldName : field name
- type: field data type
*/
