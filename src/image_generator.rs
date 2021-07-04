use image::{RgbImage};

pub struct ImageGenerator {
    pub image: RgbImage,
    pub message: Vec<i32>
}

impl ImageGenerator {
    pub fn generate_image(mut self) {
        self.setup();
        self.place_data();
        self.image.save("qr.png").unwrap(); 
    }

    fn setup(&mut self) {
        // fill image out with white pixels
        for (_, _, pixel) in self.image.enumerate_pixels_mut() {
            *pixel = image::Rgb([255, 255, 255u8]);
        }
        self.create_basic_code()
    }

    fn create_basic_code(&mut self) {
        // add position indicators in corners
        for i in 0..7 {
            *self.image.get_pixel_mut(0, i) = image::Rgb([0, 0 , 0u8]);
            *self.image.get_pixel_mut(6, i) = image::Rgb([0, 0 , 0u8]);
            *self.image.get_pixel_mut(i, 0) = image::Rgb([0, 0 , 0u8]);
            *self.image.get_pixel_mut(i, 6) = image::Rgb([0, 0 , 0u8]);

            *self.image.get_pixel_mut(0, i+14) = image::Rgb([0, 0 , 0u8]);
            *self.image.get_pixel_mut(6, i+14) = image::Rgb([0, 0 , 0u8]);
            *self.image.get_pixel_mut(i+14, 0) = image::Rgb([0, 0 , 0u8]);
            *self.image.get_pixel_mut(i+14, 6) = image::Rgb([0, 0 , 0u8]);

            *self.image.get_pixel_mut(14, i) = image::Rgb([0, 0 , 0u8]);
            *self.image.get_pixel_mut(20, i) = image::Rgb([0, 0 , 0u8]);
            *self.image.get_pixel_mut(i, 14) = image::Rgb([0, 0 , 0u8]);
            *self.image.get_pixel_mut(i, 20) = image::Rgb([0, 0 , 0u8]);
        }
        for i in 2..5 {
            for j in 2..5 {
                *self.image.get_pixel_mut(i, j) = image::Rgb([0, 0 , 0u8]);
                *self.image.get_pixel_mut(i, j+14) = image::Rgb([0, 0 , 0u8]);
                *self.image.get_pixel_mut(i+14, j) = image::Rgb([0, 0 , 0u8]);
            }
        }
    
        // add timing indicators
        for i in 8..13 {
            if i % 2 == 0 {
                *self.image.get_pixel_mut(6, i) = image::Rgb([0, 0, 0u8]);
                *self.image.get_pixel_mut(i, 6) = image::Rgb([0, 0, 0u8]);
            } else {
                *self.image.get_pixel_mut(6, i) = image::Rgb([255, 255, 255u8]);
                *self.image.get_pixel_mut(i, 6) = image::Rgb([255, 255, 255u8]);
            }
        }
    }

    fn place_data(&mut self) {
        let message_int = self.message.to_owned();

        // convert to binary
        let mut codewords_binary= vec![];
        for part in message_int {
            codewords_binary.push(format!("{:b}", part));
        }

        // fill with leading zeros
        for part in codewords_binary.iter_mut() {
            while part.chars().count() < 8 {
                *part = "0".to_string() + part;   
            }
        }
        println!("{:?}", codewords_binary);

        for codeword in codewords_binary {
            // TODO place in image matrix
        }
    }
}