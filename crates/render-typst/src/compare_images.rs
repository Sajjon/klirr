use image_compare::Algorithm;
use log::warn;
use std::path::Path;
use std::process::Command;

/// Compares a PNG `new_image` against the file at `path_to_expected_image`.
///
/// If `allow_overwrite_expected` is `true`, the expected file is replaced with
/// the new image whenever it is missing or the comparison fails.
pub(crate) fn _compare_image_against_expected(
    new_image: Vec<u8>,
    path_to_expected_image: impl AsRef<Path>,
    allow_overwrite_expected: bool,
) {
    assert!(
        is_imagemagick_installed(),
        "Imagemagick not installed, but required to run. `brew install imagemagick`"
    );
    let expected_path = path_to_expected_image.as_ref().to_path_buf();

    let save_new_image_as_expected = |bytes: &[u8]| {
        if allow_overwrite_expected {
            std::fs::write(&expected_path, bytes).unwrap_or_else(|error| {
                panic!("failed to write image to {:?}: {}", &expected_path, error)
            });
        }
    };

    let new_image_rgb = image::load_from_memory(&new_image)
        .expect("could convert new image bytes to image")
        .into_rgb8();

    let expected_image = match image::open(&expected_path) {
        Ok(image) => image.into_rgb8(),
        Err(error) => {
            warn!(
                "Failed to locate the expected image at {:?} ({}). Saving new image as expected (if allowed).",
                expected_path, error
            );
            save_new_image_as_expected(&new_image);
            return;
        }
    };

    let comparison_result = image_compare::rgb_similarity_structure(
        &Algorithm::RootMeanSquared,
        &new_image_rgb,
        &expected_image,
    );

    if let Err(failure) = &comparison_result {
        save_new_image_as_expected(&new_image);
        panic!(
            "Failed to compare images, did you change DPI or the image format? Image compare error: {:?} (replacing expected image).",
            failure
        );
    }

    let similarity = comparison_result.expect("Checked Err above");

    if similarity.score != 1.0 {
        save_new_image_as_expected(&new_image);
        panic!(
            "Expected similarity to be 1.0, but was {} (replacing expected image)",
            similarity.score
        );
    }
}

/// Checks if ImageMagick is installed by trying to run `magick -version` or `convert -version`.
pub(crate) fn is_imagemagick_installed() -> bool {
    Command::new("magick").arg("-version").output().is_ok()
        || Command::new("convert").arg("-version").output().is_ok()
}
