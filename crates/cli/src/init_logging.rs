use chrono::Local;
use colored::{ColoredString, Colorize};
use log::{Level, debug};
use std::str::FromStr;

const RUST_LOG_ENV: &str = "RUST_LOG";

fn color_from_level(level: Level) -> ColoredString {
    match level {
        log::Level::Error => "ERROR".red(),
        log::Level::Warn => "WARN".yellow(),
        log::Level::Info => "INFO".green(),
        log::Level::Debug => "DEBUG".blue(),
        log::Level::Trace => "TRACE".white(),
    }
}

/// # Panics
/// Panics if `log_level` is not a valid log level.
pub(crate) fn init_logging_with_level(log_level: log::LevelFilter) {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let time = Local::now().format("%H:%M:%S%.3f");
            let color = color_from_level(record.level());
            out.finish(format_args!("{time} {color} > {message}"));
        })
        .level(log_level)
        .chain(std::io::stdout())
        .apply()
        .inspect_err(|e| println!("💥 Failed to initialize logging with level `{log_level}`: {e}"))
        .unwrap();

    if let Some(log_level) = log_level.to_level() {
        debug!(
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
    if let Ok(log_level) = std::env::var(RUST_LOG_ENV) {
        init_logging_with_level_str(&log_level);
    } else {
        init_logging_with_level(log::LevelFilter::Info);
    }
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

    use super::*;

    #[test]
    #[should_panic(expected = "")]
    fn invalid_log_level() {
        init_logging_with_level_str("foobar");
    }

    #[test]
    fn test_color_from_level() {
        assert_eq!(
            color_from_level(Level::Error).to_string(),
            "ERROR".red().to_string()
        );
        assert_eq!(
            color_from_level(Level::Warn).to_string(),
            "WARN".yellow().to_string()
        );
        assert_eq!(
            color_from_level(Level::Info).to_string(),
            "INFO".green().to_string()
        );
        assert_eq!(
            color_from_level(Level::Debug).to_string(),
            "DEBUG".blue().to_string()
        );
        assert_eq!(
            color_from_level(Level::Trace).to_string(),
            "TRACE".white().to_string()
        );
    }
}
