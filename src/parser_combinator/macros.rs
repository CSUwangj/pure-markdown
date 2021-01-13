#[macro_export]
macro_rules! take_except {
    ($i:expr, $fun:expr) => {
        $crate::parser_combinator::take_except($fun)($i)
    };
}
