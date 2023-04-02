/// Macro for line number
/// Usage: f!("message")
/// Note: f!() is a macro that adds the line number to the message
#[macro_export]
macro_rules! f {
    ( $( $arg:tt )* ) => {{
        format!( "({} L{}) {}", file!(), line!(), format!( $( $arg )* ) )
    }}
}

/// Macro for logging info
/// Usage: info!("message")
#[macro_export]
macro_rules! info {
    ( $( $arg:tt )* ) => {{
        Logging::info(f!( $( $arg )* ))
    }}
}

/// Macro for logging error
/// Usage: error!("message")
#[macro_export]
macro_rules! error {
    ( $( $arg:tt )* ) => {{
        Logging::error(f!( $( $arg )* ))
    }}
}

/// Macro for logging debug
/// Usage: debug!("message")
#[macro_export]
macro_rules! debug {
    ( $( $arg:tt )* ) => {{
        Logging::debug(f!( $( $arg )* ))
    }}
}

/// Test macro
#[cfg(test)]
mod tests {
    #[test]
    fn test_f() {
        let message = f!("test {}", "test");
        assert_eq!(message, "(src\\macros.rs L44) test test"); // 16 is the line number of this line
    }
}
