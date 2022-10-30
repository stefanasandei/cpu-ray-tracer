use image::{ImageBuffer, RgbImage};

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub data: RgbImage,
}

pub fn create(width: u32, height: u32) -> Framebuffer {
    let fb = Framebuffer {
        width: width,
        height: height,
        data: ImageBuffer::new(width, height),
    };

    return fb;
}

impl Framebuffer {
    pub fn serialize(&self, name: &'static str) {
        let filename = format!("renders/{}.png", name);
        self.data
            .save_with_format(filename, image::ImageFormat::Png)
            .unwrap();
    }
}
