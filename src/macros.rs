#[macro_export]
macro_rules! clone_local {
    ( $( $x:ident ),* ) => {
        $(let $x = $x.clone();)*
    }
}
