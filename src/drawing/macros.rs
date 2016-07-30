#[macro_export]
macro_rules! call_rgba_fn {
    ($obj:expr, $func:ident, $color:expr) => {
        {
            let Color(r,g,b,a) = $color;
            $obj.$func(r, g, b, a);
        }
    };
    ($obj:expr, $func:ident, $arg:expr, $color:expr) => {
        {
            let Color(r,g,b,a) = $color;
            $obj.$func($arg, r, g, b, a);
        }
    };
}
