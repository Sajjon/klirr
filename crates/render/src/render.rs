use crate::prelude::*;
use invoice_typst_logic::prelude::to_typst_let;
use typst::layout::PagedDocument;
use typst_pdf::PdfOptions;
use typst_pdf::pdf;

/// Renders a PDF document using Typst with the provided layout, localization, and data.
pub fn render(layout_path: impl AsRef<Path>, l18n: L18n, data: DataTypstCompat) -> Result<Pdf> {
    let l18n_typst_str = to_typst_let(&l18n.content());
    let data_typst_str = to_typst_let(&data);
    let data_count = data_typst_str.len();

    debug!("☑️ Creating typst 'World' (environment/context), this usually takes ~2 seconds.");
    let context = TypstContext::with_path(layout_path.as_ref(), l18n_typst_str, data_typst_str)?;
    debug!("✅ Created typst 'World' (environment/context)");

    debug!(
        "☑️ Compiling typst source at: {:?} with data: #{} bytes",
        layout_path.as_ref(),
        data_count
    );
    let compile_result = typst::compile::<PagedDocument>(&context);
    let doc = compile_result.output.map_err(|e| {
        let msg = format!("Failed to compile Typst source, because: {:?}", e);
        error!("{}", msg);
        Error::BuildPdf { underlying: msg }
    })?;
    debug!(
        "✅ Finished compiling typst source, output is #{} pages",
        doc.pages.len()
    );
    // Export the PagedDocument to a PDF file.
    let export_pdf_options = &PdfOptions::default();
    let pdf_bytes = pdf(&doc, export_pdf_options).map_err(|e| {
        let msg = format!("Failed to export PDF, because: {:?}", e);
        error!("{}", msg);
        Error::ExportDocumentToPdf { underlying: msg }
    })?;
    // Convert the exported PDF bytes into a Pdf type.
    let pdf = Pdf::from(pdf_bytes);
    Ok(pdf)
}

/// Resolves a path relative to the crate this function is defined in.
///
/// The base is the folder containing this crate’s `Cargo.toml`.
#[cfg(test)]
pub fn path_to_resource(relative: impl AsRef<Path>) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative)
}

#[cfg(test)]
pub fn is_imagemagick_installed() -> bool {
    // Try `magick` (modern) first
    if std::process::Command::new("magick")
        .arg("-version")
        .output()
        .is_ok()
    {
        return true;
    }

    // Fallback to `convert` (legacy)
    if std::process::Command::new("convert")
        .arg("-version")
        .output()
        .is_ok()
    {
        return true;
    }

    false
}

/// Resolves a path relative to the crate this function is defined in.
///
/// The base is the folder containing this crate’s `Cargo.toml`.
#[cfg(test)]
pub fn fixture(relative: impl AsRef<Path>) -> PathBuf {
    path_to_resource("fixtures").join(relative)
}

#[cfg(test)]
mod tests {
    use super::*;
    use magick_rust::{LayerMethod, MagickWand, magick_wand_genesis};
    use std::{env, sync::Once};
    use test_log::test;

    fn running_in_ci() -> bool {
        env::var("CI").is_ok()
    }

    #[test]
    fn test_sample_expenses() {
        if running_in_ci() {
            // Skip this test in CI, as it requires imagemagick to be installed.
            return;
        }
        compare_image_against_expected(
            DataFromDisk::sample(),
            ValidInput::builder()
                .items(InvoicedItems::Expenses)
                .month(YearAndMonth::sample())
                .language(Language::EN)
                .build(),
            fixture("expected_expenses.png"),
        );
    }

    #[test]
    fn test_sample_services() {
        if running_in_ci() {
            // Skip this test in CI, as it requires imagemagick to be installed.
            return;
        }
        compare_image_against_expected(
            DataFromDisk::sample(),
            ValidInput::builder()
                .items(InvoicedItems::Service { days_off: None })
                .month(YearAndMonth::sample())
                .language(Language::EN)
                .build(),
            fixture("expected_services.png"),
        );
    }

    fn compare_image_against_expected(
        sample: DataFromDisk,
        input: ValidInput,
        path_to_expected_image: impl AsRef<Path>,
    ) {
        assert!(
            is_imagemagick_installed(),
            "Imagemagick not installed, but required to run. `brew install imagemagick`"
        );
        let new_image = generate_pdf_into_png_image(
            path_to_resource("src/invoice.typ"),
            L18n::new(Language::EN).unwrap(),
            sample,
            input,
        );

        let save_new_image_as_expected = |new_image: Vec<u8>| {
            if !running_in_ci() {
                // save newly produced image as expected.
                std::fs::write(&path_to_expected_image, new_image)
                    .inspect_err(|e| {
                        panic!(
                            "should be able to write image, got error: {:?}, when using path: {:?}",
                            e,
                            path_to_expected_image.as_ref()
                        )
                    })
                    .unwrap();
            }
        };

        let image_one = image::load_from_memory(&new_image)
            .expect("Could convert new image bytes to image")
            .into_rgb8();
        let Ok(image_two) = image::open(&path_to_expected_image) else {
            warn!(
                "Failed to locate the expected image at {:?}, saving new image as expected (if not CI).",
                path_to_expected_image.as_ref()
            );
            save_new_image_as_expected(new_image);
            return;
        };

        let image_two = image_two.into_rgb8();
        let comparison_result = image_compare::rgb_similarity_structure(
            &image_compare::Algorithm::RootMeanSquared,
            &image_one,
            &image_two,
        );
        if let Err(failure) = comparison_result {
            let msg = format!(
                "Failed to compare images, did you change DPI or the image format? Image compare error: {:?}",
                failure
            );
            save_new_image_as_expected(new_image);
            panic!("{}, replacing expected image.", msg);
        };
        let similarity = comparison_result.expect("Already checked for error above");

        if similarity.score != 1.0 {
            save_new_image_as_expected(new_image);

            panic!(
                "Expected similarity to be 1.0, but was {}",
                similarity.score
            )
        }
    }

    fn generate_pdf_into_png_image(
        layout_path: impl AsRef<Path>,
        l18n: L18n,
        sample: DataFromDisk,
        input: ValidInput,
    ) -> Vec<u8> {
        let data =
            prepare_invoice_input_data(sample, input, Some(ExchangeRates::hard_coded())).unwrap();
        let pdf = render(layout_path, l18n, data).unwrap();
        convert_pdf_to_pngs(pdf.as_ref(), 85.0).expect("Should be able to convert")
    }

    static INIT: Once = Once::new();

    use std::error::Error;

    /// Converts PDF bytes to a single PNG image, with a white background and correct size.
    ///
    /// # Arguments
    /// * `pdf_bytes` - A slice of bytes representing the PDF file.
    /// * `dpi` - Resolution in dots per inch (e.g., 300 for high quality)
    ///
    /// # Returns
    /// Result containing the number of pages converted or an error.
    fn convert_pdf_to_pngs(pdf_bytes: &[u8], dpi: f64) -> Result<Vec<u8>, Box<dyn Error>> {
        // Initialize MagickWand environment once
        INIT.call_once(|| {
            magick_wand_genesis();
        });

        // Create a new MagickWand instance
        let mut wand = MagickWand::new();

        // Set resolution before reading the PDF (e.g., 300 DPI for better quality)
        wand.set_resolution(dpi, dpi)?;

        // Read PDF bytes into the wand
        wand.read_image_blob(pdf_bytes)?;

        // Get the number of pages
        let num_pages = wand.get_number_images();

        assert_eq!(num_pages, 1);

        // Set the current image in the sequence
        wand.set_iterator_index(0)?;

        // Get page dimensions (in pixels, based on DPI)
        let width = wand.get_image_width();
        let height = wand.get_image_height();

        // Set the background color to white
        let mut background = magick_rust::PixelWand::new();
        background.set_color("white")?;
        wand.set_image_background_color(&background)?;

        // Extend the canvas to ensure the background covers the full area
        wand.extend_image(width, height, 0, 0)?;

        // Flatten the image to apply the white background
        wand.merge_image_layers(LayerMethod::Flatten)?;

        // Set the output format to PNG
        wand.set_format("PNG")?;

        // Get the image blob (PNG data)
        let png_data = wand.write_image_blob("PNG")?;

        Ok(png_data)
    }
}
