#[macro_export]
macro_rules! days {
    ( $($x:expr),*) => {
        paste::paste! {
            $(
                mod [<day $x>];
            )*
        }
        paste::paste! {
            pub const DAYS: &[fn()] = &[
                $(
                    [<day $x>]::main,
                )*
            ];
        }
    };
}
