extern crate image;
extern crate lcs_diff;
extern crate base64;
#[macro_use]
extern crate clap;

mod diff;
mod image_creator;
mod mkdir;
mod rename;

use std::cmp;
use image::*;
use image_creator::*;
use diff::*;
use rename::*;
use clap::{App, Arg};

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("before_image")
                 .help("path to before image")
                 .required(true))
        .arg(Arg::with_name("after_image")
                 .help("path to after image")
                 .required(true))
        .arg(Arg::with_name("diff_image")
                 .help("path to diff image")
                 .required(true));
    let matches = app.get_matches();
    let before_image = matches.value_of("before_image").unwrap();
    let after_image = matches.value_of("after_image").unwrap();
    let diff_image = matches.value_of("diff_image").unwrap();
    let mut before = image::open(before_image).unwrap();
    let mut after = image::open(after_image).unwrap();
    let compare_before = CompareImage::new(before.dimensions(), before.raw_pixels());
    let compare_after = CompareImage::new(after.dimensions(), after.raw_pixels());
    let result = diff(compare_before, compare_after);
    let rate: f32 = 100.0 / 256.0;

    let mut added: Vec<usize> = Vec::new();
    let mut removed: Vec<usize> = Vec::new();
    for d in result.iter() {
        match d {
            &lcs_diff::DiffResult::Added(ref a) => added.push(a.new_index.unwrap()),
            &lcs_diff::DiffResult::Removed(ref r) => removed.push(r.old_index.unwrap()),
            _ => (),
        }
    }
    let marked_before = add_prefix_to_file_name(&before_image, &"marked_");
    let marked_after = add_prefix_to_file_name(&after_image, &"marked_");
    save_marked_org_image(&marked_before, &mut before, (255, 0, 0), rate, &removed);
    save_marked_org_image(&marked_after, &mut after, (0, 255, 0), rate, &added);
    let width = cmp::max(before.dimensions().0, after.dimensions().0);
    save_diff_image(diff_image, width, &result, rate);
}