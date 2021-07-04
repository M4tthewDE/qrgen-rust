use image::{ImageBuffer, RgbImage};

pub struct ImageGenerator {
}

impl ImageGenerator {
    pub fn generate_image(self) {
        let mut buf: RgbImage = ImageBuffer::new(21, 21);
        self.setup(&mut buf);
        buf.save("qr.png").unwrap(); 
    }

    fn setup(self, buf: &mut RgbImage) {
        // fill image out with white pixels
        for (_, _, pixel) in buf.enumerate_pixels_mut() {
            *pixel = image::Rgb([255, 255, 255u8]);
        }
        self.create_basic_code(buf)
    }

    fn create_basic_code(self, buf: &mut RgbImage) {
        // add position indicators in corners
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
    
        // add timing indicators
        for i in 8..13 {
            if i % 2 == 0 {
                *buf.get_pixel_mut(6, i) = image::Rgb([0, 0, 0u8]);
                *buf.get_pixel_mut(i, 6) = image::Rgb([0, 0, 0u8]);
            } else {
                *buf.get_pixel_mut(6, i) = image::Rgb([255, 255, 255u8]);
                *buf.get_pixel_mut(i, 6) = image::Rgb([255, 255, 255u8]);
            }
        }
    }
}