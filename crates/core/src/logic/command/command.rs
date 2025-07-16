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

fn input_data_at<Period: IsPeriod + Serialize>(
    default_data: Data<Period>,
    write_path: impl AsRef<Path>,
    provide_data: impl FnOnce(Data<Period>) -> Result<Data<Period>>,
) -> Result<()> {
    let data = provide_data(default_data)?;
    save_data_with_base_path(data, write_path)?;
    Ok(())
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
    provide_data: impl FnOnce(Data<PeriodAnno>) -> Result<Data<PeriodAnno>>,
) -> Result<()> {
    let path = path.as_ref();
    info!("Editing data at: {}", path.display());
    let existing = read_data_from_disk_with_base_path(path)?;
    input_data_at(existing, path, provide_data)?;
    info!("✅ Data edit done");
    Ok(())
}

pub fn init_data_at<Period: IsPeriod + Serialize + DeserializeOwned + HasSample>(
    write_path: impl AsRef<Path>,
    provide_data: impl FnOnce(Data<Period>) -> Result<Data<Period>>,
) -> Result<()> {
    let write_path = write_path.as_ref();
    info!("Initializing data directory at: {}", write_path.display());
    input_data_at(Data::<Period>::sample(), write_path, provide_data)?;
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
    path: impl AsRef<Path>,
    mutate: impl FnOnce(&mut D),
) -> Result<()> {
    let mut data = deserialize_contents_of_ron(&path)?;
    mutate(&mut data);
    save_to_disk(&data, &path)?;
    Ok(())
}

/// Adds `expenses` to the specified `period` in the data file at `data_path`.
///
/// # Throws
/// Throws an error if the period type is incompatible with the service fees cadence.
pub fn record_expenses_with_base_path<Period: IsPeriod + Serialize + DeserializeOwned>(
    period: &Period,
    expenses: &[Item],
    data_path: impl AsRef<Path>,
) -> Result<()> {
    let data_path = data_path.as_ref();
    info!("Recording #{} expenses for: {:?}", expenses.len(), period);

    // First we assert that we are not mixing months and fortnights
    let service_fees = service_fees(data_path)?;
    match (
        service_fees.cadence(),
        Into::<PeriodAnno>::into(period.clone()),
    ) {
        (Cadence::Monthly, PeriodAnno::YearMonthAndFortnight(_)) => {
            return Err(Error::CannotExpenseForFortnightWhenCadenceIsMonthly);
        }
        (Cadence::BiWeekly, PeriodAnno::YearAndMonth(_)) => {
            return Err(Error::CannotExpenseForMonthWhenCadenceIsBiWeekly);
        }
        (Cadence::Monthly, PeriodAnno::YearAndMonth(_)) => {
            // Monthly cadence is compatible with YearAndMonth
        }
        (Cadence::BiWeekly, PeriodAnno::YearMonthAndFortnight(_)) => {
            // BiWeekly cadence is compatible with YearMonthAndFortnight
        }
    }

    mutate(
        expensed_periods_path(data_path),
        |data: &mut ExpensedPeriods<Period>| {
            data.insert_expenses(period, expenses.to_vec());
        },
    )
    .inspect(|_| {
        info!("✅ Expenses recorded successfully");
    })
}

pub fn record_period_off_with_base_path<Period: IsPeriod + Serialize + DeserializeOwned>(
    period: &Period,
    base_path: impl AsRef<Path>,
) -> Result<()> {
    info!("Recording period off for: {:?}", period);
    mutate(
        proto_invoice_info_path(base_path),
        |data: &mut ProtoInvoiceInfo<Period>| data.insert_period_off(period.clone()),
    )
    .inspect(|_| {
        info!("✅ Period off recorded successfully");
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
        save_data_with_base_path(Data::<YearAndMonth>::sample(), tempdir.path()).unwrap();
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
        let result = init_data_at::<YearAndMonth>(tempdir.path(), Ok);
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
            &ProtoInvoiceInfo::<YearAndMonth>::sample(),
            proto_invoice_info_path(tempdir.path()),
        )
        .unwrap();
        record_period_off_with_base_path(&month, tempdir.path()).unwrap();

        // Verify that the month was recorded correctly
        let data = proto_invoice_info(tempdir.path()).unwrap();
        assert!(data.record_of_periods_off().contains(&month));
    }

    #[test]
    fn test_record_expenses_with_base_path() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        save_to_disk(&ServiceFees::sample(), service_fees_path(tempdir.path())).unwrap();
        save_to_disk(
            &ExpensedPeriods::<YearAndMonth>::sample(),
            expensed_periods_path(tempdir.path()),
        )
        .unwrap();
        let month = YearAndMonth::may(2025);
        let expenses = vec![Item::sample_expense_breakfast()];

        record_expenses_with_base_path(&month, &expenses, tempdir.path()).unwrap();

        // Verify that the month was recorded correctly
        let data = expensed_periods::<YearAndMonth>(tempdir.path()).unwrap();
        assert!(data.contains(&month));
    }

    #[test]
    fn test_record_expenses_with_base_path_fail_because_wrong_period_kind() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        save_to_disk(
            &ExpensedPeriods::<YearAndMonth>::sample(),
            expensed_periods_path(tempdir.path()),
        )
        .unwrap();
        let period = YearMonthAndFortnight::builder()
            .year(2025.into())
            .month(Month::May)
            .half(MonthHalf::Second)
            .build();
        let expenses = vec![Item::sample_expense_breakfast()];

        let result = record_expenses_with_base_path(&period, &expenses, tempdir.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_record_expenses_with_base_path_fail_because_wrong_period_kind_ymf() {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        save_to_disk(
            &ExpensedPeriods::<YearMonthAndFortnight>::sample(),
            expensed_periods_path(tempdir.path()),
        )
        .unwrap();
        let period = YearAndMonth::may(2025);
        let expenses = vec![Item::sample_expense_breakfast()];

        let result = record_expenses_with_base_path(&period, &expenses, tempdir.path());
        assert!(result.is_err());
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
        let data = Data::<YearAndMonth>::sample();
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
            deserialize_contents_of_ron(email_settings_path(tempdir.path())).unwrap();
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

    #[test]
    fn test_record_expenses_with_base_path_throws_when_cadence_is_monthly_and_period_is_fortnight()
    {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        let period = YearMonthAndFortnight::builder()
            .year(Year::from(2025))
            .month(Month::January)
            .half(MonthHalf::First)
            .build();
        let services_fees: ServiceFees = ServiceFees::builder()
            .cadence(Cadence::Monthly)
            .rate(Rate::daily(UnitPrice::ONE))
            .name("Sample Service Fees".to_owned())
            .build()
            .unwrap();
        save_to_disk(&services_fees, service_fees_path(tempdir.path())).unwrap();
        let result = record_expenses_with_base_path(
            &period,
            &[Item::sample_expense_breakfast()],
            tempdir.path(),
        );
        assert!(
            result.is_err(),
            "Expected error when recording expenses for fortnight with monthly cadence, got: {:?}",
            result
        );
    }

    #[test]
    fn test_record_expenses_with_base_path_throws_when_cadence_is_bi_weekly_and_period_is_year_and_month()
     {
        let tempdir = tempfile::tempdir().expect("Failed to create temp dir");
        let period = YearAndMonth::may(2025);
        let services_fees: ServiceFees = ServiceFees::builder()
            .cadence(Cadence::BiWeekly)
            .rate(Rate::daily(UnitPrice::ONE))
            .name("Sample Service Fees".to_owned())
            .build()
            .unwrap();
        save_to_disk(&services_fees, service_fees_path(tempdir.path())).unwrap();
        let result = record_expenses_with_base_path(
            &period,
            &[Item::sample_expense_breakfast()],
            tempdir.path(),
        );
        assert!(
            result.is_err(),
            "Expected error when recording expenses for year and month with bi-weekly cadence, got: {:?}",
            result
        );
    }
}
