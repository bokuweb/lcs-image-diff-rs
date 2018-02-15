use super::base64::encode;
use super::lcs_diff;

#[derive(Debug)]
pub struct CompareImage {
    dimensions: (u32, u32),
    pixels: Vec<u8>,
}

impl CompareImage {
    pub fn new(dimensions: (u32, u32), pixels: Vec<u8>) -> Self {
        CompareImage { dimensions, pixels }
    }

    pub fn create_encoded_rows(&self) -> Vec<String> {
        self.pixels
            .chunks(self.dimensions.0 as usize * 4)
            .map(|chunk| encode(chunk))
            .collect()
    }
}

pub fn diff(imga: CompareImage, imgb: CompareImage) -> Vec<lcs_diff::DiffResult<String>> {
    let imga = imga.create_encoded_rows();
    let imgb = imgb.create_encoded_rows();
    lcs_diff::diff(&imga, &imgb)
}