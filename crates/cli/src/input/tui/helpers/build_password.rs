use rpassword::prompt_password;
use secrecy::{ExposeSecret as _, SecretString};

use crate::prelude::*;

fn validate(input: SecretString, min_length: usize) -> Result<SecretString> {
    let length = input.expose_secret().len();
    if length < min_length {
        Err(Error::EmailPasswordTooShort {
            min_length,
            actual_length: length,
        })
    } else {
        Ok(input)
    }
}

pub fn ask_for_password_once_with_length(
    prompt: &str,
    help: &str,
    min_length: usize,
    show_min_length: bool,
) -> Result<SecretString> {
    // Password from `read_password` will be zeroize at end of this function,
    // see https://github.com/conradkleinespel/rooster/pull/50/files
    let maybe_min_length_str = if show_min_length {
        format!(", min: #{} letters.", min_length)
    } else {
        "".to_owned()
    };
    prompt_password(format!("{} ({}{})", prompt, help, maybe_min_length_str))
        .map(SecretString::from)
        .map_err(|e| Error::InvalidPasswordForEmail {
            purpose: prompt.to_string(),
            underlying: e.to_string(),
        })
        .and_then(curry2(validate, min_length))
}

const PASSWORD_MIN_LENGTH: usize = 4;

fn ask_for_password_once(prompt: &str, help: &str, show_min_length: bool) -> Result<SecretString> {
    ask_for_password_once_with_length(prompt, help, PASSWORD_MIN_LENGTH, show_min_length)
}

pub fn ask_for_password(with_confirmation: bool, prompt: &str, help: &str) -> Result<SecretString> {
    let first = ask_for_password_once(prompt, help, with_confirmation)?;
    if !with_confirmation {
        return Ok(first);
    }
    let second = ask_for_password_once("Confirm password", help, with_confirmation)?;
    if first.expose_secret() != second.expose_secret() {
        return Err(Error::PasswordDoesNotMatch);
    }
    // second will be zeroized on drop
    Ok(first)
}

const ENV_VAR_KLIRR_EMAIL_ENCRYPTION_PASSWORD: &str = "KLIRR_EMAIL_ENCRYPTION_PASSWORD";

/// Tries to read `KLIRR_EMAIL_ENCRYPTION_PASSWORD` env variable first, if not
/// set or not at least 4 chars fallback to TUI
pub fn ask_for_email_encryption_password_with_confirmation(
    with_confirmation: bool,
) -> Result<SecretString> {
    if let Ok(env_pw) = std::env::var(ENV_VAR_KLIRR_EMAIL_ENCRYPTION_PASSWORD) {
        if env_pw.len() >= PASSWORD_MIN_LENGTH {
            return Ok(SecretString::from(env_pw));
        }
    }
    ask_for_password(
        with_confirmation,
        "Encryption Password",
        "Used to encrypt the SMTP App Password",
    )
}

pub fn get_email_encryption_password() -> Result<SecretString> {
    ask_for_email_encryption_password_with_confirmation(false)
}
