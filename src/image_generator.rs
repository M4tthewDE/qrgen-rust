use image::{RgbImage};

pub struct ImageGenerator {
    pub image: RgbImage,
    pub data_codewords: Vec<i32>,
    pub error_codewords: Vec<i32>,
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
        let message_int = self.data_codewords.to_owned();
        *self.image.get_pixel_mut(18, 20) = image::Rgb([255, 0, 0u8]);
        *self.image.get_pixel_mut(18, 19) = image::Rgb([255, 0, 0u8]);
        *self.image.get_pixel_mut(18, 18) = image::Rgb([255, 0, 0u8]);
        *self.image.get_pixel_mut(18, 17) = image::Rgb([255, 0, 0u8]);
        *self.image.get_pixel_mut(17, 20) = image::Rgb([255, 0, 0u8]);
        *self.image.get_pixel_mut(17, 19) = image::Rgb([255, 0, 0u8]);
        *self.image.get_pixel_mut(17, 18) = image::Rgb([255, 0, 0u8]);
        *self.image.get_pixel_mut(17, 17) = image::Rgb([255, 0, 0u8]);

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

        let mut test: u32 = 0;
        for (i, pos) in CODEWORD_POSITIONS {
            for coords in pos {
                println!("{:?}", coords);
               *self.image.get_pixel_mut(coords.0, coords.1) = image::Rgb([test as u8, 0, 0]);
            }
            test += 30;
        }
    }
}
const CODEWORD_POSITIONS: [(usize, [(u32,u32); 8]); 26] = [
    // column 1
    (1, [(20,20), (20,19), (20,18), (20,17), (19,20), (19,19), (19,18), (19,17)]),
    (1, [(20,16), (20,15), (20,14), (20,13), (19,16), (19,15), (19,14), (19,13)]),
    (1, [(20,12), (20,11), (20,10), (20,9), (19,12), (19,11), (19,10), (19,9)]),
    // column 2
    (1, [(18,20), (18,19), (18,18), (18,17), (17,20), (17,19), (17,18), (17,17)]),
    (1, [(18,16), (18,15), (18,14), (18,13), (17,16), (17,15), (17,14), (17,13)]),
    (1, [(18,12), (18,11), (18,10), (18,9), (17,12), (17,11), (17,10), (17,9)]),
    // column 3
    (1, [(16,20), (16,19), (16,18), (16,17), (15,20), (15,19), (15,18), (15,17)]),
    (1, [(16,16), (16,15), (16,14), (16,13), (15,16), (15,15), (15,14), (15,13)]),
    (1, [(16,12), (16,11), (16,10), (16,9), (15,12), (15,11), (15,10), (15,9)]),
    // column 4
    (1, [(14,20), (14,19), (14,18), (14,17), (13,20), (13,19), (13,18), (13,17)]),
    (1, [(14,16), (14,15), (14,14), (14,13), (13,16), (13,15), (13,14), (13,13)]),
    (1, [(14,12), (14,11), (14,10), (14,9), (13,12), (13,11), (13,10), (13,9)]),
    // column 5
    (1, [(12,20), (12,19), (12,18), (12,17), (11,20), (11,19), (11,18), (11,17)]),
    (1, [(12,16), (12,15), (12,14), (12,13), (11,16), (11,15), (11,14), (11,13)]),
    (1, [(12,12), (12,11), (12,10), (12,9), (11,12), (11,11), (11,10), (11,9)]),
    (1, [(12,8), (12,7), (12,5), (12,4), (11,8), (11,7), (11,5), (11,4)]),
    (1, [(12,3), (12,2), (12,1), (12,0), (11,3), (11,2), (11,1), (11,0)]),
    // column 6
    (1, [(10,20), (10,19), (10,18), (10,17), (9,20), (9,19), (9,18), (9,17)]),
    (1, [(10,16), (10,15), (10,14), (10,13), (9,16), (9,15), (9,14), (9,13)]),
    (1, [(10,12), (10,11), (10,10), (10,9), (9,12), (9,11), (9,10), (9,9)]),
    (1, [(10,8), (10,7), (10,5), (10,4), (9,8), (9,7), (9,5), (9,4)]),
    (1, [(10,3), (10,2), (10,1), (10,0), (9,3), (9,2), (9,1), (9,0)]),
    // column 7
    (1, [(8,12), (8,11), (8,10), (8,9), (7,12), (7,11), (7,10), (7,9)]),
    // column 8
    (1, [(5,12), (5,11), (5,10), (5,9), (4,12), (4,11), (4,10), (4,9)]),
    // column 9
    (1, [(3,12), (3,11), (3,10), (3,9), (2,12), (2,11), (2,10), (2,9)]),
    // column 10
    (1, [(1,12), (1,11), (1,10), (1,9), (0,12), (0,11), (0,10), (0,9)]),
];