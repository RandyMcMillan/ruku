use colored::Colorize;

/// Used for reporting Docker build information to stdout.
pub struct Logger {}

impl Logger {
    pub fn new() -> Logger {
        Logger {}
    }

    /// Pretty-print the given log section title.
    pub fn section(&self, msg: &str) {
        println!("=== {} ===", msg.magenta().bold());
    }

    /// Pretty-print the given log line.
    pub fn step(&self, msg: &str) {
        println!("=> {}", msg.cyan());
    }

    /// Pretty-print error message
    pub fn error(&self, msg: &str) {
        eprintln!("=> {}", msg.red());
    }

    /// Pretty-print the given log line as a warning and exit the app.
    pub fn fatal(&self, msg: &str) {
        eprintln!("=> {}", msg.red());
        std::process::exit(1);
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}
