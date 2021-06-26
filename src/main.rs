extern crate image;

use image::{ImageBuffer, RgbImage};

fn main() {
    generate_image()
}

fn generate_image() {
    let input = "12345";

    let buf: RgbImage = ImageBuffer::new(21, 21);
    setup(buf);

    encode_data(input);
}

fn setup(mut buf: RgbImage) {
    // fill image out with white pixels
    for (_, _, pixel) in buf.enumerate_pixels_mut() {
        *pixel = image::Rgb([255, 255, 255 as u8]);
    }
    add_pos_patterns(buf)
}

fn add_pos_patterns(mut buf: RgbImage) {
    for i in 0..7 {
        *buf.get_pixel_mut(0, i) = image::Rgb([0, 0 , 0 as u8]);
        *buf.get_pixel_mut(6, i) = image::Rgb([0, 0 , 0 as u8]);
        *buf.get_pixel_mut(i, 0) = image::Rgb([0, 0 , 0 as u8]);
        *buf.get_pixel_mut(i, 6) = image::Rgb([0, 0 , 0 as u8]);

        *buf.get_pixel_mut(0, i+14) = image::Rgb([0, 0 , 0 as u8]);
        *buf.get_pixel_mut(6, i+14) = image::Rgb([0, 0 , 0 as u8]);
        *buf.get_pixel_mut(i+14, 0) = image::Rgb([0, 0 , 0 as u8]);
        *buf.get_pixel_mut(i+14, 6) = image::Rgb([0, 0 , 0 as u8]);

        *buf.get_pixel_mut(0+14, i) = image::Rgb([0, 0 , 0 as u8]);
        *buf.get_pixel_mut(6+14, i) = image::Rgb([0, 0 , 0 as u8]);
        *buf.get_pixel_mut(i, 0+14) = image::Rgb([0, 0 , 0 as u8]);
        *buf.get_pixel_mut(i, 6+14) = image::Rgb([0, 0 , 0 as u8]);
    }
    for i in 2..5 {
        for j in 2..5 {
            *buf.get_pixel_mut(i, j) = image::Rgb([0, 0 , 0 as u8]);
            *buf.get_pixel_mut(i, j+14) = image::Rgb([0, 0 , 0 as u8]);
            *buf.get_pixel_mut(i+14, j) = image::Rgb([0, 0 , 0 as u8]);
        }
    }

    buf.save("qr.png").unwrap(); 
}

fn encode_data(input: &str) {
    let mode = "0001";
    let mut length = format!("{:b}", input.chars().count());              

    while length.chars().count() < 10 {
        length = "0".to_string() + &length;        
    }

    let mut result = vec![];
    result.push(mode);
    result.push(&length);

    let chars: Vec<char> = input.chars().collect();
    let mut data_str = vec![];

    // split in parts of 3
    let mut tmp = String::from("");
    for (pos, c) in chars.iter().enumerate() {
        tmp.push(*c);
        if ((pos+1) % 3 == 0) && (pos != 0) {
            data_str.push(tmp.to_string());            
            tmp.clear();
        }
        if pos == input.chars().count()-1 {
            data_str.push(tmp.to_string());
        }
    }

    // convert to binary
    let mut data = vec![];
    for part in data_str {
        data.push(format!("{:b}", part.parse::<i32>().unwrap()));
    }

    // fill with leading zeroes
    for part in data.iter_mut() {
        let tmp_int = isize::from_str_radix(&part, 2).unwrap();
        if tmp_int.to_string().chars().count() == 3 {
            while part.chars().count() < 10 {
                *part = "0".to_string() + &part;   
            }
        }
        if tmp_int.to_string().chars().count() == 2 {
            while part.chars().count() < 7 {
                *part = "0".to_string() + &part;
            }
        }
        if tmp_int.to_string().chars().count() == 1 {
            while part.chars().count() < 4 {
                *part = "0".to_string() + &part;
            }
        }
    }

    // put it all together
    for part in data.iter() {
        result.push(part);
    }

    // add terminator
    result.push("0000");

    println!("End: {:?}", result);
}