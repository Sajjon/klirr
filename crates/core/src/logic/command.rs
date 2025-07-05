use crate::prelude::*;
use secrecy::SecretString;
use serde::de::DeserializeOwned;

fn input_email_data_at(
    write_path: impl AsRef<Path>,
    provide_data: impl FnOnce() -> Result<EncryptedEmailSettings>,
) -> Result<()> {
    let email_settings = provide_data()?;
    save_email_settings_with_base_path(email_settings, write_path)?;
    Ok(())
}

fn input_data_at(
    default_data: Data,
    write_path: impl AsRef<Path>,
    provide_data: impl FnOnce(Data) -> Result<Data>,
) -> Result<()> {
    let data = provide_data(default_data)?;
    save_data_with_base_path(data, write_path)?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataSelector {
    /// All but expensed months
    All,
    Vendor,
    Client,
    Information,
    PaymentInfo,
    ServiceFees,
}

impl DataSelector {
    pub fn includes(&self, target: DataSelector) -> bool {
        match self {
            DataSelector::All => true,
            DataSelector::Vendor => matches!(target, DataSelector::Vendor),
            DataSelector::Client => matches!(target, DataSelector::Client),
            DataSelector::Information => matches!(target, DataSelector::Information),
            DataSelector::PaymentInfo => matches!(target, DataSelector::PaymentInfo),
            DataSelector::ServiceFees => matches!(target, DataSelector::ServiceFees),
        }
    }
}

pub fn edit_data_at(
    path: impl AsRef<Path>,
    provide_data: impl FnOnce(Data) -> Result<Data>,
) -> Result<()> {
    let path = path.as_ref();
    info!("Editing data at: {}", path.display());
    let existing = read_data_from_disk_with_base_path(path)?;
    input_data_at(existing, path, provide_data)?;
    info!("✅ Data edit done");
    Ok(())
}

pub fn init_data_at(
    write_path: impl AsRef<Path>,
    provide_data: impl FnOnce(Data) -> Result<Data>,
) -> Result<()> {
    let write_path = write_path.as_ref();
    info!("Initializing data directory at: {}", write_path.display());
    input_data_at(Data::sample(), write_path, provide_data)?;
    info!("✅ Data init done, you're ready: `{} invoice`", BINARY_NAME);
    Ok(())
}

fn decrypt_email_settings_and<T>(
    read_path: impl AsRef<Path>,
    ask_for_email_password: impl FnOnce() -> Result<SecretString>,
    on_decrypt: impl FnOnce(DecryptedEmailSettings) -> Result<T>,
) -> Result<T> {
    let read_path = read_path.as_ref();
    let email_settings: EncryptedEmailSettings =
        load_data(read_path, DATA_FILE_NAME_EMAIL_SETTINGS)?;
    let encryption_password = ask_for_email_password()?;
    let email_settings = email_settings.decrypt_smtp_app_password(encryption_password)?;
    on_decrypt(email_settings)
}

impl From<(DecryptedEmailSettings, NamedPdf)> for Email {
    fn from((settings, pdf): (DecryptedEmailSettings, NamedPdf)) -> Self {
        let (subject, body) = settings.proto_email().materialize(pdf.prepared_data());
        Email::builder()
            .subject(subject)
            .body(body)
            .public_recipients(settings.public_recipients().clone())
            .cc_recipients(settings.cc_recipients().clone())
            .bcc_recipients(settings.bcc_recipients().clone())
            .attachments([Attachment::Pdf(pdf)])
            .build()
    }
}

impl From<DecryptedEmailSettings> for EmailCredentials {
    fn from(settings: DecryptedEmailSettings) -> Self {
        EmailCredentials::builder()
            .account(
                EmailAccount::builder()
                    .name(settings.sender().name())
                    .email(settings.sender().email().clone())
                    .build(),
            )
            .password(settings.smtp_app_password())
            .smtp_server(settings.smtp_server().clone())
            .build()
    }
}

impl DecryptedEmailSettings {
    pub fn compose(&self, pdf: NamedPdf) -> (Email, EmailCredentials) {
        let email = Email::from((self.clone(), pdf));
        let credentials = EmailCredentials::from(self.clone());
        (email, credentials)
    }
}

fn load_email_data_and_send_test_email_at_with_send(
    read_path: impl AsRef<Path>,
    ask_for_email_password: impl FnOnce() -> Result<SecretString>,
    render_sample: impl FnOnce() -> Result<NamedPdf>,
    send_email: impl FnOnce(Email, EmailCredentials) -> Result<()>,
) -> Result<()> {
    let read_path = read_path.as_ref();
    info!(
        "Loading email settings for sending test email from: {}",
        read_path.display()
    );
    decrypt_email_settings_and(read_path, ask_for_email_password, |email_settings| {
        let sample = render_sample()?;
        let (email, credentials) = email_settings.compose(sample);
        send_email(email, credentials)
            .inspect(|_| info!("Email sent successfully!"))
            .inspect_err(|e| {
                error!("Error sending email: {}", e);
            })
    })
}

pub fn load_email_data_and_send_test_email_at(
    read_path: impl AsRef<Path>,
    ask_for_email_password: impl FnOnce() -> Result<SecretString>,
    render_sample: impl FnOnce() -> Result<NamedPdf>,
) -> Result<()> {
    load_email_data_and_send_test_email_at_with_send(
        read_path,
        ask_for_email_password,
        render_sample,
        send_email_with_credentials,
    )
}

pub fn validate_email_data_at(
    read_path: impl AsRef<Path>,
    ask_for_email_password: impl FnOnce() -> Result<SecretString>,
) -> Result<()> {
    let read_path = read_path.as_ref();
    info!("Validating email settings at: {}", read_path.display());
    decrypt_email_settings_and(read_path, ask_for_email_password, |email_settings| {
        info!(
            "✅ Email settings validated successfully, ready to send emails from: {} using #{} characters long app password",
            email_settings.sender().email(),
            email_settings.smtp_app_password().len()
        );
        Ok(())
    })
}

pub fn init_email_data_at(
    write_path: impl AsRef<Path>,
    provide_data: impl FnOnce() -> Result<EncryptedEmailSettings>,
) -> Result<()> {
    let write_path = write_path.as_ref();
    info!(
        "Initializing email settings directory at: {}",
        write_path.display()
    );
    input_email_data_at(write_path, provide_data)?;
    info!("✅ Data init done, you're ready: `{} invoice`", BINARY_NAME);
    Ok(())
}

fn mutate<D: Serialize + DeserializeOwned + Clone>(
    data_path: impl AsRef<Path>,
    data_file_name: &str,
    mutate: impl FnOnce(&mut D),
) -> Result<()> {
    let data_path = data_path.as_ref();
    let mut data = load_data::<D>(data_path, data_file_name)?.clone();
    mutate(&mut data);
    let path = path_to_ron_file_with_base(data_path, data_file_name);
    save_to_disk(&data, path)?;
    Ok(())
}

pub fn record_expenses_with_base_path(
    month: &YearAndMonth,
    expenses: &[Item],
    data_path: impl AsRef<Path>,
) -> Result<()> {
    info!("Recording #{} expenses for: {}", expenses.len(), month);
    mutate(
        data_path,
        DATA_FILE_NAME_EXPENSES,
        |data: &mut ExpensedMonths| {
            data.insert_expenses(month, expenses.to_vec());
        },
    )
    .inspect(|_| {
        info!("✅ Expenses recorded successfully");
    })
}

pub fn record_month_off_with_base_path(
    month: &YearAndMonth,
    data_path: impl AsRef<Path>,
) -> Result<()> {
    info!("Recording month off for: {}", month);
    mutate(
        data_path,
        DATA_FILE_NAME_PROTO_INVOICE_INFO,
        |data: &mut ProtoInvoiceInfo| {
            data.insert_month_off(*month);
        },
    )
    .inspect(|_| {
        info!("✅ Month off recorded successfully");
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn save_to_disk_err_serialize() {
        use serde::{self, Serialize, Serializer};
        struct FailModel;

        impl Serialize for FailModel {
            fn serialize<S>(&self, _serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                Err(serde::ser::Error::custom(
                    "manual failure during serialization",
                ))
            }
        }

        let fail_model = FailModel;
        let result = save_to_disk(&fail_model, PathBuf::from("irrelevant"));
        assert!(result.is_err(), "Expected save to fail, got: {:?}", result);
    }

    #[test]
    fn save_to_disk_err_invalid_path() {
        let result = save_to_disk(
            &CompanyInformation::sample_client(),
            PathBuf::from("/invalid/path"),
        );
        assert!(result.is_err(), "Expected save to fail, got: {:?}", result);
    }

    #[test]
    fn test_read_data_from_disk() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        save_data_with_base_path(Data::sample(), tempdir.path()).unwrap();
        let result = read_data_from_disk_with_base_path(tempdir.path());
        assert!(
            result.is_ok(),
            "Expected validation to succeed, got: {:?}",
            result
        );
    }

    #[test]
    fn test_init_data_directory_at() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        let result = init_data_at(tempdir.path(), Ok);
        assert!(
            result.is_ok(),
            "Expected data directory initialization to succeed, got: {:?}",
            result
        );
    }

    #[test]
    fn test_record_month_off_with_base_path() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        let month = YearAndMonth::may(2025);
        save_to_disk(
            &ProtoInvoiceInfo::sample(),
            path_to_ron_file_with_base(tempdir.path(), DATA_FILE_NAME_PROTO_INVOICE_INFO),
        )
        .unwrap();
        record_month_off_with_base_path(&month, tempdir.path()).unwrap();

        // Verify that the month was recorded correctly
        let data = proto_invoice_info(tempdir.path()).unwrap();
        assert!(data.months_off_record().contains(&month));
    }

    #[test]
    fn test_record_expenses_with_base_path() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        save_to_disk(
            &ExpensedMonths::sample(),
            path_to_ron_file_with_base(tempdir.path(), DATA_FILE_NAME_EXPENSES),
        )
        .unwrap();
        let month = YearAndMonth::may(2025);
        let expenses = vec![Item::sample_expense_breakfast()];

        record_expenses_with_base_path(&month, &expenses, tempdir.path()).unwrap();

        // Verify that the month was recorded correctly
        let data = expensed_months(tempdir.path()).unwrap();
        assert!(data.contains(&month));
    }

    #[test]
    fn test_data_selector_includes() {
        let all_selector = DataSelector::All;
        assert!(all_selector.includes(DataSelector::All));
        assert!(all_selector.includes(DataSelector::Vendor));
        assert!(all_selector.includes(DataSelector::Client));
        assert!(all_selector.includes(DataSelector::Information));
        assert!(all_selector.includes(DataSelector::PaymentInfo));
        assert!(all_selector.includes(DataSelector::ServiceFees));

        let vendor_selector = DataSelector::Vendor;
        assert!(vendor_selector.includes(DataSelector::Vendor));
        assert!(!vendor_selector.includes(DataSelector::Client));

        let selector = DataSelector::Client;
        assert!(selector.includes(DataSelector::Client));
        assert!(!selector.includes(DataSelector::Vendor));
        assert!(!selector.includes(DataSelector::All));

        let selector = DataSelector::Information;
        assert!(selector.includes(DataSelector::Information));
        assert!(!selector.includes(DataSelector::Vendor));
        assert!(!selector.includes(DataSelector::All));

        let selector = DataSelector::PaymentInfo;
        assert!(selector.includes(DataSelector::PaymentInfo));
        assert!(!selector.includes(DataSelector::Vendor));
        assert!(!selector.includes(DataSelector::All));

        let selector = DataSelector::ServiceFees;
        assert!(selector.includes(DataSelector::ServiceFees));
        assert!(!selector.includes(DataSelector::Vendor));
        assert!(!selector.includes(DataSelector::All));
    }

    #[test]
    fn test_edit_data_at() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        let data = Data::sample();
        let first = CompanyInformation::sample_vendor();
        let second = CompanyInformation::sample_client();
        assert_ne!(
            first, second,
            "Sample vendor and client should not be the same"
        );
        save_data_with_base_path(data.with_client(first.clone()), tempdir.path()).unwrap();
        let result = edit_data_at(tempdir.path(), |data| Ok(data.with_client(second.clone())));
        assert!(
            result.is_ok(),
            "Expected data edit to succeed, got: {:?}",
            result
        );
        let edited_data = read_data_from_disk_with_base_path(tempdir.path()).unwrap();
        assert_eq!(*edited_data.client(), second);
    }

    #[test]
    fn test_input_email_data_at() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        let email_settings = EncryptedEmailSettings::sample();
        let result = input_email_data_at(tempdir.path(), || Ok(email_settings.clone()));
        assert!(
            result.is_ok(),
            "Expected email data input to succeed, got: {:?}",
            result
        );
        let loaded_email_settings: EncryptedEmailSettings =
            load_data(tempdir.path(), DATA_FILE_NAME_EMAIL_SETTINGS).unwrap();
        assert_eq!(email_settings, loaded_email_settings);
    }

    #[test]
    fn test_validate_email_data_at() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        let email_settings = EncryptedEmailSettings::sample();
        input_email_data_at(tempdir.path(), || Ok(email_settings.clone())).unwrap();
        let result = validate_email_data_at(tempdir.path(), || Ok(SecretString::sample()));
        assert!(
            result.is_ok(),
            "Expected email data validation to succeed, got: {:?}",
            result
        );
    }

    #[test]
    fn test_load_email_data_and_send_test_email_at_with_send() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        let email_settings = EncryptedEmailSettings::sample();
        input_email_data_at(tempdir.path(), || Ok(email_settings.clone())).unwrap();

        let result = load_email_data_and_send_test_email_at_with_send(
            tempdir.path(),
            || Ok(SecretString::sample()),
            || Ok(NamedPdf::sample()),
            |email, credentials| {
                assert!(!email.subject().is_empty());
                assert!(!credentials.account().email().user().is_empty());
                Ok(())
            },
        );
        assert!(
            result.is_ok(),
            "Expected email sending to succeed, got: {:?}",
            result
        );
    }
}
