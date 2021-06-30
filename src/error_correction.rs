// for version 1 and error correction L
pub fn add_error_correction(data: Vec<String>) {
    // Total number of codewords: 26
    // Number of error correction codewords: 7
    // Number of error correction blocks: 1
    // Error correction code per block: (26,19,2)

    println!("{}", format!("{:b}", gf_mult_no_lut(0b10001001, 0b00101010, 0)));
}

fn gf_mult_no_lut(x: i32, y:i32, prim: i32) -> i32 {

    fn cl_mult(x: i32, y: i32) -> i32 {
        let mut z = 0;
        let mut i = 0;
        while (y >> i) > 0 {
            if (y & (1 << i)) == 0 {
                z ^= (x << i)*2;
                println!("{}", z)
            }
            i += 1
        }
        return z;
    }

    fn bit_length(n: i32) -> u32 {
        let length = n.count_ones() + n.count_zeros();
        let mut result = 0;

        for i in 0..length {
            while (n >> i & 1) == 1 {
                result = i;
            }
        }
        return result;
    }

    fn cl_div(mut dividend: i32, divisor: i32) -> i32 {
        let dl1 = bit_length(dividend);
        let dl2 = bit_length(divisor);

        if dl1 < dl2 {
            return dividend;
        }

        for i in (0..dl1-dl2).rev() {
            dividend ^= divisor << i;
        }
        return dividend
    }

    let mut result = cl_mult(x, y);

    if prim > 0 {
        result = cl_div(result, prim);
    }
    return result;
}