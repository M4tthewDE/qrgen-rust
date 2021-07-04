// reference: https://www.swisseduc.ch/informatik/theoretische_informatik/qr_codes/docs/qr_standard.pdf

extern crate image;

mod error_correction;
mod image_generator;
use error_correction::{build_correction_calculator, CorrectionCalculator};
use image_generator::ImageGenerator;
use image::ImageBuffer;

use std::i32;

fn main() {
    let input = "12345";
    let message = encode_data(input);

    let image_generator = ImageGenerator{
        image: ImageBuffer::new(21, 21),
        message: message
    };
    image_generator.generate_image();
}

fn encode_data(input: &str) -> Vec<i32> {
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

    codeword_conversion(&result)
}

fn codeword_conversion(data: &str) -> Vec<i32> {

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

    // Total number of codewords: 26
    // Number of data codewords: 19
    // Number of error correction codewords: 7s
    // Number of error correction blocks: 1
    // Error correction code per block: (26,19,2)
    let correction_calculator: CorrectionCalculator = build_correction_calculator();
    
    // convert strings to i32s
    let mut data_codewords = vec!();
    for e in data_str.iter() {
        data_codewords.push(i32::from_str_radix(e, 2).unwrap());
    }

    let error_correction_codwords = correction_calculator.rs_encode_msg(data_codewords.to_owned(), 7);
    construct_final_message(data_codewords, error_correction_codwords)
}

fn construct_final_message(mut data_codewords: Vec<i32>, mut error_correction_codewords: Vec<i32>) -> Vec<i32> {
    data_codewords.append(&mut error_correction_codewords);
    data_codewords
}