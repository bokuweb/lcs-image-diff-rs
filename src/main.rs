extern crate futures;
extern crate futures_cpupool;
extern crate image;
extern crate lcs_image_diff;

#[macro_use]
extern crate clap;

mod mkdir;
mod rename;

use std::path::Path;
use image::*;
use rename::*;
use mkdir::*;
use clap::{App, Arg};
use futures_cpupool::CpuPool;
use futures::{future, Future};
use lcs_image_diff::compare;

static RATE: f32 = 100.0 / 256.0;

pub fn save_image(image: &DynamicImage, filename: &str) {
    let path = Path::new(filename).parent().unwrap();
    let _result = mkdirp(path);
    image.save(filename).unwrap();
}

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("before_image")
                .help("path to before image")
                .required(true),
        )
        .arg(
            Arg::with_name("after_image")
                .help("path to after image")
                .required(true),
        )
        .arg(
            Arg::with_name("diff_image")
                .help("path to diff image")
                .required(true),
        );
    let matches = app.get_matches();
    let before_image = matches.value_of("before_image").unwrap();
    let after_image = matches.value_of("after_image").unwrap();
    let diff_image = matches.value_of("diff_image").unwrap().to_owned();
    let mut before = image::open(before_image).unwrap();
    let mut after = image::open(after_image).unwrap();

    let marked_before = add_prefix_to_file_name(&before_image, &"marked_");
    let marked_after = add_prefix_to_file_name(&after_image, &"marked_");

    let result = compare(&mut before, &mut after, RATE);

    {
        let thread_pool = CpuPool::new_num_cpus();
        let before_thread = thread_pool.spawn_fn(move || -> Result<(), ()> {
            save_image(&before, &marked_before);
            Ok(())
        });
        let after_thread = thread_pool.spawn_fn(move || -> Result<(), ()> {
            save_image(&after, &marked_after);
            Ok(())
        });
        let result_thread = thread_pool.spawn_fn(move || -> Result<(), ()> {
            save_image(&result, &diff_image);
            Ok(())
        });
        future::join_all(vec![before_thread, after_thread, result_thread])
            .wait()
            .unwrap();
    }
}
