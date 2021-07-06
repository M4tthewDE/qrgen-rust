use image::{RgbImage};

const FORMAT_VERSION_POSITIONS: [(usize, [(u32,u32); 8]); 4] = [
    // top left
    (1, [(0,8), (1,8), (2,8), (3,8), (4,8), (5,8), (7,8), (8,8)]),
    (1, [(8,0), (8,1), (8,2), (8,3), (8,4), (8,5), (8,8), (8,7)]),
    // bottom left
    (1, [(8,13), (8,14), (8,15), (8,16), (8,17), (8,18), (8,19), (8,20)]),
    // top right
    (1, [(13,8), (14,8), (15,8), (16,8), (17,8), (18,8), (19,8), (20,8)]),
];

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
        let mut message_int = self.data_codewords.to_owned();
        message_int.append(&mut self.error_codewords.to_owned());

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
        for (_, pos) in FORMAT_VERSION_POSITIONS {
            for coords in pos {
               *self.image.get_pixel_mut(coords.0, coords.1) = image::Rgb([0, 255, 0]);
            }
        }

        /*
        // color pixels to see which ones were not affected by actions
        for pos in CODEWORD_POSITIONS {
            for coords in pos {
               *self.image.get_pixel_mut(coords.0, coords.1) = image::Rgb([255, 0, 0]);
            }
        }

        // put in codewords
        for (i, codeword) in codewords_binary.iter().enumerate() {
            let direction = &CODEWORD_POSITIONS[i].1;
            let positions = CODEWORD_POSITIONS[i].0.to_vec().to_owned();
            for (j, bit) in codeword.chars().enumerate() {
                match (bit) {
                    '0' => { 
                        *self.image.get_pixel_mut(positions[j].0, positions[j].1) = image::Rgb([0, 0, 0]);
                                                            
                    }
                    '1' => {
                        *self.image.get_pixel_mut(positions[j].0, positions[j].1) = image::Rgb([255, 255, 255]);
                    }
                    _ => {panic!("how did it even get here");}
                }
            }
        }
        */
    }
}