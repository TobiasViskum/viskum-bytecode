#[macro_export]
macro_rules! debug_features {
    ($($item:stmt)*) => {
        #[cfg(any(feature = "debug_trace_execution", feature = "debug_print_code"))]
        $($item)*
    };
}

#[macro_export]
macro_rules! debug_trace_execution {
    ($($block:block)*) => {
        #[cfg(any(feature = "debug_trace_execution", feature = "debug_print_code"))]
        $($block)*
    };
}

#[macro_export]
macro_rules! debug_print_code {
    ($($item:stmt)*) => {
        #[cfg(any(feature = "debug_trace_execution", feature = "debug_print_code"))]
        $($item)*
    };
}
