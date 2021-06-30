// reference: https://www.swisseduc.ch/informatik/theoretische_informatik/qr_codes/docs/qr_standard.pdf

extern crate image;

mod error_correction;

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
        *pixel = image::Rgb([255, 255, 255u8]);
    }
    add_pos_patterns(buf)
}

fn add_pos_patterns(mut buf: RgbImage) {
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

fn encode_data(input: &str) {
    let mode = "0001";
    let mut length = format!("{:b}", input.chars().count());              

    while length.chars().count() < 10 {
        length = "0".to_string() + &length;        
    }

    let mut result_arr = vec![mode, &length];

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

    // fill with leading zeros
    for part in data.iter_mut() {
        let tmp_int = isize::from_str_radix(&part, 2).unwrap();
        if tmp_int.to_string().chars().count() == 3 {
            while part.chars().count() < 10 {
                *part = "0".to_string() + part;   
            }
        }
        if tmp_int.to_string().chars().count() == 2 {
            while part.chars().count() < 7 {
                *part = "0".to_string() + part;
            }
        }
        if tmp_int.to_string().chars().count() == 1 {
            while part.chars().count() < 4 {
                *part = "0".to_string() + part;
            }
        }
    }

    // put it all together
    for part in data.iter() {
        result_arr.push(part);
    }

    // add terminator
    // note: this is only needed 
    // if the sequence does not fill out the entire available space!
    // TODO: check if needed!
    result_arr.push("0000");

    // concat elements
    let mut result = String::from("");
    for e in result_arr {
        result += e;
    }

    codeword_conversion(&result);
}

fn codeword_conversion(data: &str) {

    // divide in parts with length=8
    let mut data_str = vec![];
    let mut tmp = String::from("");
    for (pos, c) in data.chars().enumerate() {
        tmp.push(c);
        if ((pos+1) % 8 == 0) && (pos != 0) {
            data_str.push(tmp.to_string());            
            tmp.clear();
        }
        if pos == data.len()-1 {
            data_str.push(tmp.to_string());
        }
    }

    // add padding bits to last element if it's too short
    match data_str.last_mut() {
        Some(x) => {
            if x.len() < 8 {
                while x.chars().count() < 8 {
                    x.push('0');
                }
            }
        },
        None => println!("Empty data!"),
    }    

    // add Pad Codewords 11101100 and 00010001 alternately
    // 1. find out how many bits are used up
    let mut bits_used = 0;
    for e in data_str.iter() {
        bits_used += e.len();
    }
     
    // 2. calculate how many Pad Codewords need to be added
    // -> depends on the version and error correction level
    // -> we are using version 1 and error correction leve L to start out
    let pad_codewords_amount = (152-bits_used)/8;

    // 3. add Pad Codewords alternately
    for i in 0..pad_codewords_amount {
        if i % 2 == 0 {
            data_str.push("11101100".to_string());            
        } else {
            data_str.push("00010001".to_string());            
        }
    }
    error_correction::add_error_correction(data_str);
}