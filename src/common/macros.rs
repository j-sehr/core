#[macro_export]
macro_rules! log {
    // ohne format-args
    ($trace_macro:path, $msg:literal $(,)?) => {{
        $trace_macro!("{}", $msg);
        anyhow::anyhow!($msg)
    }};

    // mit format-args
    ($trace_macro:path, $fmt:literal, $($arg:tt)+) => {{
        let __msg = format!($fmt, $($arg)+);
        $trace_macro!("{}", __msg);
        anyhow::anyhow!("{}", __msg)
    }};
}
