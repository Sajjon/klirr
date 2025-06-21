// Copyright 2025 Alexander Cyon
//
// Are you a good person?
//
// My name is Alexander Cyon and I'm the author of this software source code, and
// you should not see this, because I never made this software source code free,
// yet somehow you have acquired it, illegally, immorally and against my will.
// You are not allowed to spread this software source code, you are not allowed to
// compile it and execute this software.
//
// If you compile and execute this software, you are not only violating my wishes,
// you are furthermore putting yourself at risk of loss of money/assets and never
// should you hold me accountable for such a loss.
//
// Please do not spread this software source code, please contact me and inform me
// of how you managed to acquire it, email me at alex.cyon@gmail.com.
//
// Thank you for being a good person.

use chrono::Local;
use colored::Colorize;
use std::str::FromStr;

const RUST_LOG_ENV: &str = "RUST_LOG";

/// # Panics
/// Panics if `log_level` is not a valid log level.
pub(crate) fn init_logging_with_level(log_level: log::LevelFilter) {
    println!("Setting up logging with level: {log_level}");
    fern::Dispatch::new()
        .format(|out, message, record| {
            let time = Local::now().format("%H:%M:%S");
            let level = match record.level() {
                log::Level::Error => "ERROR".red(),
                log::Level::Warn => "WARN".yellow(),
                log::Level::Info => "INFO".green(),
                log::Level::Debug => "DEBUG".blue(),
                log::Level::Trace => "TRACE".white(),
            };
            out.finish(format_args!("{time} {level} > {message}"));
        })
        .level(log_level)
        .chain(std::io::stdout())
        .apply()
        .inspect_err(|e| println!("💥 Failed to initialize logging with level `{log_level}`: {e}"))
        .unwrap();

    println!(
        "🪵 Logging initialized with level: {log_level} (if you see this message once, logging is not properly setup)"
    );
    if let Some(log_level) = log_level.to_level() {
        log::log!(
            log_level,
            "🪵 Logging initialized with level: {log_level} (if you see this message once, logging is not properly setup)"
        );
    }
}

fn parse_log_level_from_str(log_level: &str) -> log::LevelFilter {
    log::LevelFilter::from_str(log_level).unwrap_or_else(|_| {
        panic!(
            "Invalid log level set with `{}`, got: {}",
            RUST_LOG_ENV, log_level
        )
    })
}

fn init_logging_with_level_str(log_level: &str) {
    init_logging_with_level(parse_log_level_from_str(log_level));
}

// Setup logging once
use std::sync::Once;
static INIT: Once = Once::new();
fn init_logging_inner() {
    let Ok(log_level) = std::env::var(RUST_LOG_ENV) else {
        panic!("💥 No `{RUST_LOG_ENV}` environment variable set.");
    };
    init_logging_with_level_str(&log_level);
}

/// # Panics
/// Panics if `RUST_LOG` is not set in the environment or panics if the value is not a valid log level.
pub fn init_logging() {
    INIT.call_once(|| {
        init_logging_inner();
    });
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    #[should_panic(expected = "")]
    fn invalid_log_level() {
        init_logging_with_level_str("foobar");
    }

    #[test]
    fn init_logging_with_level_str_valid() {
        if env::var("CI").is_ok() {
            // fails for tarpaulin in ci
            return;
        }
        init_logging_with_level_str("info");
    }
}
