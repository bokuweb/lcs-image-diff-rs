use std::cmp;
use super::image::*;
use super::lcs_diff::*;
use super::base64::decode;

pub static BLACK: (u8, u8, u8) = (0, 0, 0);
pub static RED: (u8, u8, u8) = (255, 119, 119);
pub static GREEN: (u8, u8, u8) = (99, 195, 99);

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
where
    G: GenericImage<Pixel = Rgba<u8>>,
{
    for (y1, y2) in ranges {
        for y in y1..(y2 + 1) {
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
        data: [
            (base.data[0] as f32 * (1.0 - rate) + rgb.0 as f32 * (rate)) as u8,
            (base.data[1] as f32 * (1.0 - rate) + rgb.1 as f32 * (rate)) as u8,
            (base.data[2] as f32 * (1.0 - rate) + rgb.2 as f32 * (rate)) as u8,
            base.data[3],
        ],
    };
}

fn put_diff_pixels(
    y: usize,
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    row_width: u32,
    data: &String,
    rgb: (u8, u8, u8),
    rate: f32,
) {
    let row = decode(data).unwrap();
    for x in 0..img.dimensions().0 {
        let index = x as usize * 4;
        let pixel: Rgba<u8> = if row_width > x {
            Rgba {
                data: [row[index], row[index + 1], row[index + 2], row[index + 3]],
            }
        } else {
            Rgba { data: [0, 0, 0, 0] }
        };
        img.put_pixel(x as u32, y as u32, blend(pixel, rgb, rate));
    }
}

pub fn mark_org_image(
    base: &mut DynamicImage,
    color: (u8, u8, u8),
    rate: f32,
    indexes: &Vec<usize>,
) {
    let range = compute_range(indexes);
    blend_diff_area(base, range, color, rate);
}

pub fn get_diff_image(
    before_width: u32,
    after_width: u32,
    result: &Vec<DiffResult<String>>,
    rate: f32,
) -> DynamicImage {
    let height = result.len() as u32;
    let width = cmp::max(before_width, after_width);
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    for (y, d) in result.iter().enumerate() {
        match d {
            &DiffResult::Added(ref a) => {
                put_diff_pixels(y, &mut img, after_width, &a.data, GREEN, rate)
            }
            &DiffResult::Removed(ref r) => {
                put_diff_pixels(y, &mut img, before_width, &r.data, RED, rate)
            }
            &DiffResult::Common(ref c) => {
                put_diff_pixels(y, &mut img, width, &c.data, BLACK, 0.0)
            }
        }
    }
    ImageRgba8(img)
}
