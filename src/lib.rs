extern crate base64;
extern crate image;
extern crate lcs_diff;

mod diff;
mod image_creator;

use diff::*;
use image_creator::*;
use image::*;
pub use base64::DecodeError;

/// Accepts two mutable references to `image::DynamicImage` and rate.
/// Returns diff `image::DynamicImage` and marks removed and added
/// parts on input images.
///
/// # Examples
///
/// ```no_run
/// extern crate image;
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<Error>> {
/// use lcs_image_diff::compare;
///
/// let mut before = image::open("before.png")?;
/// let mut after = image::open("after.png")?;
///
/// let diff = compare(&mut before, &mut after, 100.0 / 256.0)?;
///
/// before.save("marked_before.png")?;
/// after.save("marked_after.png")?;
/// diff.save("diff.png")?;
/// # Ok(())
/// # }
/// ```
pub fn compare(
    before: &mut DynamicImage,
    after: &mut DynamicImage,
    rate: f32
) -> Result<DynamicImage, DecodeError> {
    let compare_before = CompareImage::new(before.dimensions(), before.raw_pixels());
    let compare_after = CompareImage::new(after.dimensions(), after.raw_pixels());
    let result = diff(compare_before, compare_after);

    let mut added: Vec<usize> = Vec::new();
    let mut removed: Vec<usize> = Vec::new();
    for d in result.iter() {
        match d {
            &lcs_diff::DiffResult::Added(ref a) => added.push(a.new_index.unwrap()),
            &lcs_diff::DiffResult::Removed(ref r) => removed.push(r.old_index.unwrap()),
            _ => (),
        }
    }

    mark_org_image(before, RED, rate, &removed);
    mark_org_image(after, GREEN, rate, &added);

    get_diff_image(before.dimensions().0, after.dimensions().0, &result, rate)
}
