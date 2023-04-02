use std::sync::atomic::{AtomicBool, Ordering};
static VERBOSE: AtomicBool = AtomicBool::new(false);

pub struct Logging {
}

/// Logging class
/// This class is used to log messages to the console
/// Usage: Logging::info(f!("message"));
/// Usage: Logging::error(f!("message"));
/// Usage: Logging::debug(f!("message"));
/// Note: f!() is a macro that adds the line number to the message
impl Logging {
    pub fn set_verbose(verbose: bool) {
        VERBOSE.store(verbose, Ordering::SeqCst);
    }

    fn remove_path(message: String) -> String {
        // Regex to remove the path from the message
        let re = regex::Regex::new(r"\((.*) L[0-9]+\)").unwrap();
        re.replace(&message, "").to_string()
    }

    pub fn info(message: String) {
        if !VERBOSE.load(Ordering::SeqCst) {
            println!("\x1b[32mINFO:\x1b[0m {}", Self::remove_path(message));
            return;
        }
        // Color green
        println!("\x1b[32mINFO:\x1b[0m  {message}");
    }

    pub fn error(message: String) {
        if !VERBOSE.load(Ordering::SeqCst) {
            println!("\x1b[31mERROR:\x1b[0m {}", Self::remove_path(message));
            return;
        }
        // Color red
        println!("\x1b[31mERROR:\x1b[0m {message}");
    }

    pub fn debug(message: String) {
        if !VERBOSE.load(Ordering::SeqCst) {
            return;
        }
        // Color blue
        println!("\x1b[34mDEBUG:\x1b[0m {message}");
    }
}

/// Tests
#[cfg(test)]
mod tests {
    use crate::f;

    use super::*;

    #[test]
    fn test_logging() {
        Logging::info(f!("test"));
        Logging::error(f!("test"));
        Logging::debug(f!("test"));
    }
}
