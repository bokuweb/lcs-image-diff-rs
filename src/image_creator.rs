use std::fs::File;
use std::path::Path;
use super::image::*;
use super::lcs_diff::*;
use super::mkdir::*;
use super::base64::decode;

fn compute_range(r: &Vec<usize>) -> Vec<(usize, usize)> {
    let mut i = 0;
    let mut j = 0;
    let mut acc: usize;
    let mut y1: usize;
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    while i < r.len() {
        y1 = r[i];
        acc = y1;
        i += 1;
        loop {
            if i >= r.len() {
                break;
            }
            let index = r[i];
            if acc + 1 != index {
                break;
            }
            acc = index;
            i += 1;
            j += 1;
        }
        let y2 = y1 + j;
        j = 0;
        ranges.push((y1, y2));
    }
    ranges
}

fn blend_diff_area<G>(img: &mut G, ranges: Vec<(usize, usize)>, rgb: (u8, u8, u8), rate: f32)
    where G: GenericImage<Pixel = Rgba<u8>>
{
    for (y1, y2) in ranges {
        for y in y1..y2 {
            for x in 0..img.dimensions().0 {
                let p = img.get_pixel(x, y as u32);
                let blended = blend(p, rgb, rate);
                img.put_pixel(x, y as u32, blended);
            }
        }
    }
}

fn blend(base: Rgba<u8>, rgb: (u8, u8, u8), rate: f32) -> Rgba<u8> {
    return Rgba {
               data: [(base.data[0] as f32 * (1.0 - rate) + rgb.0 as f32 * (rate)) as u8,
                      (base.data[1] as f32 * (1.0 - rate) + rgb.1 as f32 * (rate)) as u8,
                      (base.data[2] as f32 * (1.0 - rate) + rgb.2 as f32 * (rate)) as u8,
                      base.data[3]],
           };
}

fn put_diff_pixels(y: usize,
                   img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
                   data: &String,
                   rgb: (u8, u8, u8),
                   rate: f32) {
    let row = decode(data).unwrap();
    for x in 0..img.dimensions().0 {
        let index = x as usize * 4;
        let pixel: Rgba<u8> =
            Rgba { data: [row[index], row[index + 1], row[index + 2], row[index + 3]] };
        img.put_pixel(x as u32, y as u32, blend(pixel, rgb, rate));
    }
}

pub fn save_marked_org_image(filename: &str,
                             base: &mut DynamicImage,
                             color: (u8, u8, u8),
                             rate: f32,
                             indexes: &Vec<usize>) {
    let range = compute_range(indexes);
    blend_diff_area(base, range, color, rate);
    let ref mut fout = File::create(filename).unwrap();
    base.save(fout, PNG).unwrap();
}

pub fn save_diff_image(filename: &str, width: u32, result: &Vec<DiffResult<String>>, rate: f32) {
    let path = Path::new(filename).parent().unwrap();
    let _result = mkdirp(path);
    let height = result.len() as u32;
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    for (y, d) in result.iter().enumerate() {
        match d {
            &DiffResult::Added(ref a) => put_diff_pixels(y, &mut img, &a.data, (0, 255, 0), rate),
            &DiffResult::Removed(ref r) => put_diff_pixels(y, &mut img, &r.data, (255, 0, 0), rate),
            &DiffResult::Common(ref c) => put_diff_pixels(y, &mut img, &c.data, (0, 0, 0), 0.0),
        }
    }
    let ref mut fout = File::create(filename).unwrap();
    ImageRgba8(img).save(fout, PNG).unwrap();
}