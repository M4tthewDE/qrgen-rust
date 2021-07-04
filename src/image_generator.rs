use image::{ImageBuffer, RgbImage};

pub struct ImageGenerator {
}

impl ImageGenerator {
    pub fn generate_image(self) {
        let buf: RgbImage = ImageBuffer::new(21, 21);
        self.setup(buf);
    }

    fn setup(self, mut buf: RgbImage) {
        // fill image out with white pixels
        for (_, _, pixel) in buf.enumerate_pixels_mut() {
            *pixel = image::Rgb([255, 255, 255u8]);
        }
        self.add_pos_patterns(buf)
    }

    fn add_pos_patterns(self, mut buf: RgbImage) {
        for i in 0..7 {
            *buf.get_pixel_mut(0, i) = image::Rgb([0, 0 , 0u8]);
            *buf.get_pixel_mut(6, i) = image::Rgb([0, 0 , 0u8]);
            *buf.get_pixel_mut(i, 0) = image::Rgb([0, 0 , 0u8]);
            *buf.get_pixel_mut(i, 6) = image::Rgb([0, 0 , 0u8]);

            *buf.get_pixel_mut(0, i+14) = image::Rgb([0, 0 , 0u8]);
            *buf.get_pixel_mut(6, i+14) = image::Rgb([0, 0 , 0u8]);
            *buf.get_pixel_mut(i+14, 0) = image::Rgb([0, 0 , 0u8]);
            *buf.get_pixel_mut(i+14, 6) = image::Rgb([0, 0 , 0u8]);

            *buf.get_pixel_mut(14, i) = image::Rgb([0, 0 , 0u8]);
            *buf.get_pixel_mut(20, i) = image::Rgb([0, 0 , 0u8]);
            *buf.get_pixel_mut(i, 14) = image::Rgb([0, 0 , 0u8]);
            *buf.get_pixel_mut(i, 20) = image::Rgb([0, 0 , 0u8]);
        }
        for i in 2..5 {
            for j in 2..5 {
                *buf.get_pixel_mut(i, j) = image::Rgb([0, 0 , 0u8]);
                *buf.get_pixel_mut(i, j+14) = image::Rgb([0, 0 , 0u8]);
                *buf.get_pixel_mut(i+14, j) = image::Rgb([0, 0 , 0u8]);
            }
        }

        buf.save("qr.png").unwrap(); 
    }
}