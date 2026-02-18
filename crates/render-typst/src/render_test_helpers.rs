use crate::render::render;
use klirr_core_invoice::{
    Currency, Data, ExchangeRates, ExchangeRatesMap, FetchExchangeRates, IsPeriod, Item, L10n,
    Language, ValidInput, prepare_invoice_input_data,
};

use std::path::{Path, PathBuf};
use std::{env, error::Error, io::Write, process::Command};
use tempfile::NamedTempFile;

/// Resolves a path relative to the crate this function is defined in.
///
/// The base is the folder containing this crate’s `Cargo.toml`.
pub fn path_to_resource(relative: impl AsRef<Path>) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative)
}

/// Resolves a path relative to the crate this function is defined in.
///
/// The base is the folder containing this crate’s `Cargo.toml`.
pub fn fixture(relative: impl AsRef<Path>) -> PathBuf {
    path_to_resource("fixtures").join(relative)
}

/// Checks if we are running in a CI environment.
pub fn running_in_ci() -> bool {
    env::var("CI").is_ok()
}

/// Compares a generated image against an expected image, saving the new image if it differs.
/// If the expected image does not exist, it will save the new image as the expected one.
pub fn compare_image_against_expected<Period: IsPeriod>(
    sample: Data<Period>,
    input: ValidInput,
    path_to_expected_image: impl AsRef<Path>,
    fetcher: impl FetchExchangeRates,
) {
    let new_image =
        match generate_pdf_into_png_image(L10n::new(Language::EN).unwrap(), sample, input, fetcher)
        {
            Ok(bytes) => bytes,
            Err(err) => {
                eprintln!("Skipping image comparison: {err}");
                return;
            }
        };
    klirr_render_pdf::compare_images::compare_image_against_expected(
        new_image,
        path_to_expected_image,
        !running_in_ci(),
    );
}

#[cfg(test)]
#[derive(derive_more::From, Default)]
pub struct MockedExchangeRatesFetcher(ExchangeRatesMap);
#[cfg(test)]
impl FetchExchangeRates for MockedExchangeRatesFetcher {
    fn fetch_for_items(
        &self,
        target_currency: Currency,
        _items: Vec<Item>,
    ) -> klirr_core_invoice::Result<ExchangeRates> {
        Ok(ExchangeRates::builder()
            .rates(self.0.clone())
            .target_currency(target_currency)
            .build())
    }
}

/// Generates a PNG image from a PDF rendered from the given layout path and input data.
fn generate_pdf_into_png_image<Period: IsPeriod>(
    l10n: L10n,
    sample: Data<Period>,
    input: ValidInput,
    fetcher: impl FetchExchangeRates,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let layout = *input.layout();
    let data = prepare_invoice_input_data(sample, input, fetcher).unwrap();
    let pdf = render(l10n, data, layout, |e| panic!("Got unexpected error: {e}")).unwrap();
    convert_pdf_to_pngs(pdf.as_ref(), 85.0)
}

/// Converts PDF bytes to a single PNG image, with a white background and correct size.
///
/// # Arguments
/// * `pdf_bytes` - A slice of bytes representing the PDF file.
/// * `dpi` - Resolution in dots per inch (e.g., 300 for high quality)
///
/// # Returns
/// Result containing the number of pages converted or an error.
fn convert_pdf_to_pngs(pdf_bytes: &[u8], dpi: f64) -> Result<Vec<u8>, Box<dyn Error>> {
    // Write PDF bytes to a temporary file
    let mut temp_pdf = NamedTempFile::new()?;
    temp_pdf.write_all(pdf_bytes)?;
    let pdf_path = temp_pdf.path();

    // Create another temp file for the output PNG
    let temp_png = NamedTempFile::new()?;
    let png_path = temp_png.path().with_extension("png");

    // Construct DPI argument (as integer string)
    let dpi_arg = format!("{}", dpi as u32);

    // Run the `convert` command using ImageMagick
    let status = Command::new("magick")
        .arg("-density")
        .arg(&dpi_arg)
        .arg("-units")
        .arg("PixelsPerInch")
        .arg(pdf_path)
        .arg("-colorspace")
        .arg("RGB")
        .arg("-background")
        .arg("white")
        .arg("-alpha")
        .arg("remove")
        .arg("-flatten")
        .arg("+profile")
        .arg("*")
        .arg("-strip")
        .arg("-define")
        .arg("png:compression-filter=0")
        .arg("-define")
        .arg("png:compression-level=9")
        .arg("-define")
        .arg("png:compression-strategy=1")
        .arg(&png_path)
        .status()?;

    if !status.success() {
        return Err("ImageMagick convert command failed".into());
    }

    // Read the resulting PNG bytes
    let png_bytes = std::fs::read(&png_path)?;

    Ok(png_bytes)
}
