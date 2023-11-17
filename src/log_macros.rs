#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => ({
        format!("\x1b[1;31m[ERROR]\x1b[0m {}", format_args!($($arg)*))
    })
}

#[macro_export]
macro_rules! dbg {
    ($($arg:tt)*) => ({
        format!("\x1b[1;32m[DEBUG]\x1b[0m {}", format_args!($($arg)*))
    })
}
