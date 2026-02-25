mod calendar_logic;
mod document;
#[cfg(feature = "crypto")]
mod encryption;
#[cfg(feature = "exchange-rates")]
mod exchange_rates;
mod fs_utils;
mod functional;
mod models;
mod ron;
mod runtime;
mod sample;
mod traits;
mod typst_layouts;

pub use crate::calendar_logic::{
    CalendarError, CalendarResult, calculate_period_number, normalize_period_end_date_for_cadence,
    parse_period_label_for_cadence, period_end_from_relative_time, quantity_in_period,
};
pub use crate::document::{
    create_pdf_document, render_and_save_named_pdf, resolve_output_path_and_name,
};
#[cfg(feature = "crypto")]
pub use crate::encryption::{
    AesGcm256, AesGcmSealedBox, AesNonce, CryptoError, CryptoResult, EncryptedAppPassword,
    EncryptionKey, PbHkdfSha256, Salt,
};
#[cfg(feature = "exchange-rates")]
pub use crate::exchange_rates::{
    DeserializableResponse, ExchangeRateItem, ExchangeRatesError, ExchangeRatesFetcher,
    ExchangeRatesMap, get_exchange_rate_with_fetcher, get_exchange_rate_with_reqwest,
};
pub use crate::fs_utils::{create_folder_if_needed, create_folder_to_parent_of_path_if_needed};
pub use crate::functional::{ResultExt, curry1, curry2};
pub use crate::models::{
    AbstractNamedPdf, Cadence, Cost, Currency, Date, Day, Decimal, DueInDays, FontIdentifier,
    FontWeight, Granularity, ModelError, ModelResult, Month, OutputPath, PathAndName, Pdf,
    Quantity, RelativeTime, UnitPrice, Year, save_pdf,
};
pub use crate::ron::{
    RonError, deserialize_contents_of_ron, deserialize_ron_str, path_to_ron_file_with_base,
    save_to_disk, type_name,
};
pub use crate::runtime::{
    BINARY_NAME, TMP_FILE_FOR_PATH_TO_PDF_ENV, data_dir, data_dir_create_if,
    save_pdf_location_to_tmp_file,
};
pub use crate::sample::HasSample;
pub use crate::traits::{FontRequiring, ToTypst, ToTypstFn};
pub use crate::typst_layouts::{TYPST_LAYOUT_FOUNDATION, TYPST_LAYOUT_TEST};
