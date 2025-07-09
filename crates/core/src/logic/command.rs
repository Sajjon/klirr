use crate::prelude::*;
use secrecy::{ExposeSecret, SecretString};
use serde::de::DeserializeOwned;

fn input_email_data_at(
    default_data: EncryptedEmailSettings,
    write_path: impl AsRef<Path>,
    provide_data: impl FnOnce(EncryptedEmailSettings) -> Result<EncryptedEmailSettings>,
) -> Result<()> {
    let email_settings = provide_data(default_data)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmailSettingsSelector {
    All,
    AppPassword,
    EncryptionPassword,
    Template,
    SmtpServer,
    ReplyTo,
    Sender,
    Recipients,
    CcRecipients,
    BccRecipients,
}

pub trait Select {
    fn includes(&self, target: Self) -> bool;
}

impl Select for DataSelector {
    fn includes(&self, target: Self) -> bool {
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

impl EmailSettingsSelector {
    pub fn requires_encryption_password(&self) -> bool {
        use EmailSettingsSelector::*;
        match self {
            All | AppPassword | EncryptionPassword => true,
            Template | SmtpServer | ReplyTo | Sender | Recipients | CcRecipients
            | BccRecipients => false,
        }
    }
}
impl Select for EmailSettingsSelector {
    fn includes(&self, target: Self) -> bool {
        match self {
            EmailSettingsSelector::All => true,
            EmailSettingsSelector::AppPassword => {
                matches!(target, EmailSettingsSelector::AppPassword)
            }
            EmailSettingsSelector::EncryptionPassword => {
                matches!(target, EmailSettingsSelector::EncryptionPassword)
            }
            EmailSettingsSelector::Template => {
                matches!(target, EmailSettingsSelector::Template)
            }
            EmailSettingsSelector::SmtpServer => {
                matches!(target, EmailSettingsSelector::SmtpServer)
            }
            EmailSettingsSelector::ReplyTo => matches!(target, EmailSettingsSelector::ReplyTo),
            EmailSettingsSelector::Sender => matches!(target, EmailSettingsSelector::Sender),
            EmailSettingsSelector::Recipients => {
                matches!(target, EmailSettingsSelector::Recipients)
            }
            EmailSettingsSelector::CcRecipients => {
                matches!(target, EmailSettingsSelector::CcRecipients)
            }
            EmailSettingsSelector::BccRecipients => {
                matches!(target, EmailSettingsSelector::BccRecipients)
            }
        }
    }
}

pub fn edit_email_data_at(
    path: impl AsRef<Path>,
    provide_data: impl FnOnce(EncryptedEmailSettings) -> Result<EncryptedEmailSettings>,
) -> Result<()> {
    let path = path.as_ref();
    info!("Editing email data at: {}", path.display());
    let existing = read_email_data_from_disk_with_base_path(path)?;
    input_email_data_at(existing, path, provide_data)?;
    info!("✅ Email data edit done");
    Ok(())
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

pub fn init_email_data_at(
    write_path: impl AsRef<Path>,
    provide_data: impl FnOnce(EncryptedEmailSettings) -> Result<EncryptedEmailSettings>,
) -> Result<()> {
    let write_path = write_path.as_ref();
    info!(
        "Initializing email settings directory at: {}",
        write_path.display()
    );
    input_email_data_at(EncryptedEmailSettings::sample(), write_path, provide_data)?;
    info!("✅ Data init done, you're ready: `{} invoice`", BINARY_NAME);
    Ok(())
}

fn decrypt_email_settings_and<T>(
    read_path: impl AsRef<Path>,
    get_email_password: impl FnOnce() -> Result<SecretString>,
    on_decrypt: impl FnOnce(DecryptedEmailSettings) -> Result<T>,
) -> Result<T> {
    let read_path = read_path.as_ref();
    let email_settings = read_email_data_from_disk_with_base_path(read_path)?;
    let encryption_password = get_email_password()?;
    let email_settings = email_settings.decrypt_smtp_app_password(encryption_password)?;
    on_decrypt(email_settings)
}

impl From<(DecryptedEmailSettings, NamedPdf)> for Email {
    fn from((settings, pdf): (DecryptedEmailSettings, NamedPdf)) -> Self {
        let (subject, body) = settings.template().materialize(pdf.prepared_data());
        Email::builder()
            .subject(subject)
            .body(body)
            .public_recipients(settings.recipients().clone())
            .cc_recipients(settings.cc_recipients().clone())
            .bcc_recipients(settings.bcc_recipients().clone())
            .attachments(IndexSet::from([Attachment::Pdf(pdf)]))
            .build()
    }
}

impl From<DecryptedEmailSettings> for EmailCredentials {
    fn from(settings: DecryptedEmailSettings) -> Self {
        EmailCredentials::builder()
            .account(
                EmailAccount::builder()
                    .name(settings.sender().name().clone())
                    .email(settings.sender().email().clone())
                    .build(),
            )
            .password(settings.smtp_app_password().clone())
            .smtp_server(settings.smtp_server().clone())
            .build()
    }
}

impl DecryptedEmailSettings {
    pub fn compose(&self, pdf: &NamedPdf) -> (Email, EmailCredentials) {
        let email = Email::from((self.clone(), pdf.clone()));
        let credentials = EmailCredentials::from(self.clone());
        (email, credentials)
    }
}

fn load_email_data_and_send_test_email_at_with_send(
    read_path: impl AsRef<Path>,
    get_email_password: impl FnOnce() -> Result<SecretString>,
    render_sample: impl FnOnce() -> Result<NamedPdf>,
    send_email: impl FnOnce(&NamedPdf, &DecryptedEmailSettings) -> Result<()>,
) -> Result<()> {
    let read_path = read_path.as_ref();
    info!(
        "Loading email settings for sending test email from: {}",
        read_path.display()
    );
    decrypt_email_settings_and(read_path, get_email_password, |email_settings| {
        let sample = render_sample()?;
        send_email(&sample, &email_settings)
            .inspect(|_| info!("Email sent successfully!"))
            .inspect_err(|e| {
                error!("Error sending email: {}", e);
            })
    })
}

pub fn load_email_data_and_send_test_email_at(
    read_path: impl AsRef<Path>,
    get_email_password: impl FnOnce() -> Result<SecretString>,
    render_sample: impl FnOnce() -> Result<NamedPdf>,
) -> Result<()> {
    load_email_data_and_send_test_email_at_with_send(
        read_path,
        get_email_password,
        render_sample,
        send_email_with_settings_for_pdf,
    )
}

pub fn validate_email_data_at(
    read_path: impl AsRef<Path>,
    get_email_password: impl FnOnce() -> Result<SecretString>,
) -> Result<DecryptedEmailSettings> {
    let read_path = read_path.as_ref();
    info!("Validating email settings at: {}", read_path.display());
    decrypt_email_settings_and(read_path, get_email_password, |email_settings| {
        info!(
            "✅ Email settings validated successfully, ready to send emails from: {} using #{} characters long app password",
            email_settings.sender().email(),
            email_settings.smtp_app_password().expose_secret().len()
        );
        Ok(email_settings)
    })
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
    fn test_edit_email_data_at() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        let email_settings = EncryptedEmailSettings::sample();
        let first_sender = EmailAccount::sample_alice();
        let second_sender = EmailAccount::sample_bob();
        assert_ne!(
            first_sender, second_sender,
            "Sample sender emails should not be the same"
        );

        // Save initial email settings with first sender
        save_email_settings_with_base_path(
            email_settings.with_sender(first_sender.clone()),
            tempdir.path(),
        )
        .unwrap();

        // Edit the email settings to use second sender
        let result = edit_email_data_at(tempdir.path(), |email_settings| {
            Ok(email_settings.with_sender(second_sender.clone()))
        });

        assert!(
            result.is_ok(),
            "Expected email data edit to succeed, got: {:?}",
            result
        );

        // Verify that the edit was applied correctly
        let edited_email_settings =
            read_email_data_from_disk_with_base_path(tempdir.path()).unwrap();
        assert_eq!(*edited_email_settings.sender(), second_sender);
    }

    #[test]
    fn test_input_email_data_at() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        let email_settings = EncryptedEmailSettings::sample();
        let result =
            input_email_data_at(email_settings.clone(), tempdir.path(), |email_settings| {
                Ok(email_settings)
            });
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
        init_email_data_at(tempdir.path(), |email_settings| Ok(email_settings.clone())).unwrap();
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
        input_email_data_at(
            EncryptedEmailSettings::sample(),
            tempdir.path(),
            |email_settings| Ok(email_settings.clone()),
        )
        .unwrap();

        let result = load_email_data_and_send_test_email_at_with_send(
            tempdir.path(),
            || Ok(SecretString::sample()),
            || Ok(NamedPdf::sample()),
            |named_pdf, email_settings| {
                assert_eq!(named_pdf, &NamedPdf::sample());
                assert!(!email_settings.sender().email().user().is_empty());
                Ok(())
            },
        );
        assert!(
            result.is_ok(),
            "Expected email sending to succeed, got: {:?}",
            result
        );
    }

    #[test]
    fn test_requires_encryption_password() {
        assert!(EmailSettingsSelector::AppPassword.requires_encryption_password());
        assert!(EmailSettingsSelector::EncryptionPassword.requires_encryption_password());
        assert!(!EmailSettingsSelector::Template.requires_encryption_password());
        assert!(!EmailSettingsSelector::SmtpServer.requires_encryption_password());
        assert!(!EmailSettingsSelector::ReplyTo.requires_encryption_password());
        assert!(!EmailSettingsSelector::Sender.requires_encryption_password());
        assert!(!EmailSettingsSelector::Recipients.requires_encryption_password());
        assert!(!EmailSettingsSelector::CcRecipients.requires_encryption_password());
        assert!(!EmailSettingsSelector::BccRecipients.requires_encryption_password());
    }

    #[test]
    fn test_includes_for_email_selector() {
        let all_selector = EmailSettingsSelector::All;
        assert!(all_selector.includes(EmailSettingsSelector::All));
        assert!(all_selector.includes(EmailSettingsSelector::AppPassword));
        assert!(all_selector.includes(EmailSettingsSelector::EncryptionPassword));
        assert!(all_selector.includes(EmailSettingsSelector::Template));
        assert!(all_selector.includes(EmailSettingsSelector::SmtpServer));
        assert!(all_selector.includes(EmailSettingsSelector::ReplyTo));
        assert!(all_selector.includes(EmailSettingsSelector::Sender));
        assert!(all_selector.includes(EmailSettingsSelector::Recipients));
        assert!(all_selector.includes(EmailSettingsSelector::CcRecipients));
        assert!(all_selector.includes(EmailSettingsSelector::BccRecipients));

        let app_password_selector = EmailSettingsSelector::AppPassword;
        assert!(app_password_selector.includes(EmailSettingsSelector::AppPassword));
        assert!(!app_password_selector.includes(EmailSettingsSelector::EncryptionPassword));
        assert!(!app_password_selector.includes(EmailSettingsSelector::All));

        let encryption_password_selector = EmailSettingsSelector::EncryptionPassword;
        assert!(encryption_password_selector.includes(EmailSettingsSelector::EncryptionPassword));
        assert!(!encryption_password_selector.includes(EmailSettingsSelector::AppPassword));
        assert!(!encryption_password_selector.includes(EmailSettingsSelector::All));

        let template_selector = EmailSettingsSelector::Template;
        assert!(template_selector.includes(EmailSettingsSelector::Template));
        assert!(!template_selector.includes(EmailSettingsSelector::AppPassword));
        assert!(!template_selector.includes(EmailSettingsSelector::All));

        let smtp_server_selector = EmailSettingsSelector::SmtpServer;
        assert!(smtp_server_selector.includes(EmailSettingsSelector::SmtpServer));
        assert!(!smtp_server_selector.includes(EmailSettingsSelector::AppPassword));
        assert!(!smtp_server_selector.includes(EmailSettingsSelector::All));

        let reply_to_selector = EmailSettingsSelector::ReplyTo;
        assert!(reply_to_selector.includes(EmailSettingsSelector::ReplyTo));
        assert!(!reply_to_selector.includes(EmailSettingsSelector::AppPassword));
        assert!(!reply_to_selector.includes(EmailSettingsSelector::All));

        let sender_selector = EmailSettingsSelector::Sender;
        assert!(sender_selector.includes(EmailSettingsSelector::Sender));
        assert!(!sender_selector.includes(EmailSettingsSelector::AppPassword));
        assert!(!sender_selector.includes(EmailSettingsSelector::All));

        let recipients_selector = EmailSettingsSelector::Recipients;
        assert!(recipients_selector.includes(EmailSettingsSelector::Recipients));
        assert!(!recipients_selector.includes(EmailSettingsSelector::AppPassword));
        assert!(!recipients_selector.includes(EmailSettingsSelector::All));

        let cc_recipients_selector = EmailSettingsSelector::CcRecipients;
        assert!(cc_recipients_selector.includes(EmailSettingsSelector::CcRecipients));
        assert!(!cc_recipients_selector.includes(EmailSettingsSelector::AppPassword));
        assert!(!cc_recipients_selector.includes(EmailSettingsSelector::All));

        let bcc_recipients_selector = EmailSettingsSelector::BccRecipients;
        assert!(bcc_recipients_selector.includes(EmailSettingsSelector::BccRecipients));
        assert!(!bcc_recipients_selector.includes(EmailSettingsSelector::AppPassword));
        assert!(!bcc_recipients_selector.includes(EmailSettingsSelector::All));
    }

    #[test]
    fn test_from_decrypted_email_settings_and_named_pdf_for_email() {
        let email_settings = DecryptedEmailSettings::sample();
        let pdf = NamedPdf::sample();

        let email: Email = (email_settings.clone(), pdf.clone()).into();

        // Verify that the email was constructed correctly
        assert_eq!(email.public_recipients(), email_settings.recipients());
        assert_eq!(email.cc_recipients(), email_settings.cc_recipients());
        assert_eq!(email.bcc_recipients(), email_settings.bcc_recipients());

        // Verify that the PDF attachment was added
        let attachments = email.attachments();
        assert_eq!(attachments.len(), 1);
        assert!(
            matches!(attachments[0], Attachment::Pdf(ref attached_pdf) if attached_pdf == &pdf)
        );

        // Verify that subject and body are set (template materialization)
        assert!(!email.subject().is_empty());
        assert!(!email.body().is_empty());
    }

    #[test]
    fn test_from_decrypted_email_settings_for_email_credentials() {
        let email_settings = DecryptedEmailSettings::sample();

        let credentials: EmailCredentials = email_settings.clone().into();

        // Verify that the credentials were constructed correctly
        assert_eq!(credentials.account().name(), email_settings.sender().name());
        assert_eq!(
            credentials.account().email(),
            email_settings.sender().email()
        );
        assert_eq!(credentials.smtp_server(), email_settings.smtp_server());
        assert_eq!(
            credentials.password().expose_secret(),
            email_settings.smtp_app_password().expose_secret()
        );
    }

    #[test]
    fn test_compose_email_and_credentials() {
        let email_settings = DecryptedEmailSettings::sample();
        let pdf = NamedPdf::sample();

        let (email, credentials) = email_settings.compose(&pdf);

        // Verify the email was composed correctly
        assert_eq!(email.public_recipients(), email_settings.recipients());
        assert_eq!(email.cc_recipients(), email_settings.cc_recipients());
        assert_eq!(email.bcc_recipients(), email_settings.bcc_recipients());

        // Verify the PDF attachment was added
        let attachments = email.attachments();
        assert_eq!(attachments.len(), 1);
        assert!(
            matches!(attachments[0], Attachment::Pdf(ref attached_pdf) if attached_pdf == &pdf)
        );

        // Verify subject and body are set
        assert!(!email.subject().is_empty());
        assert!(!email.body().is_empty());

        // Verify the credentials were created correctly
        assert_eq!(credentials.account().name(), email_settings.sender().name());
        assert_eq!(
            credentials.account().email(),
            email_settings.sender().email()
        );
        assert_eq!(credentials.smtp_server(), email_settings.smtp_server());
        assert_eq!(
            credentials.password().expose_secret(),
            email_settings.smtp_app_password().expose_secret()
        );
    }
}
