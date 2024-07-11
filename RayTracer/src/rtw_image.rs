use image::{DynamicImage, GenericImageView};

pub struct RtwImage {
    data: Option<DynamicImage>,
}

impl RtwImage {
    pub fn open(image_filename: &str) -> Self {
        let path = String::from("assets/") + image_filename;
        let data;
        match image::open(&path) {
            Ok(img) => {
                data = Some(img);
            }
            Err(_) => {
                eprintln!("ERROR: Could not load image file '{}'.", path);
                data = None;
            }
        }

        Self { data }
    }

    pub fn width(&self) -> u32 {
        match &self.data {
            Some(img) => img.width(),
            None => 0,
        }
    }

    pub fn height(&self) -> u32 {
        match &self.data {
            Some(img) => img.height(),
            None => 0,
        }
    }

    pub fn pixel_data(&self, mut x: u32, mut y: u32) -> [u8; 3] {
        static MAGENTA: [u8; 3] = [255, 0, 255];

        match &self.data {
            Some(img) => {
                x = clamp(x, 0, img.width());
                y = clamp(y, 0, img.height());

                let rgba = img.get_pixel(x, y);
                [rgba[0], rgba[1], rgba[2]]
            }
            None => MAGENTA,
        }
    }
}

fn clamp(x: u32, low: u32, high: u32) -> u32 {
    if x < low {
        low
    } else if x < high {
        x
    } else {
        high - 1
    }
}
