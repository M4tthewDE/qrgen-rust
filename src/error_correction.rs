const PRIM: u32 = 0x11d;

pub fn build_correction_calculator() -> CorrectionCalculator {
    let mut correction_calculator = CorrectionCalculator {
        gf_log: Vec::new(),
        gf_exp: Vec::new()
    };
    correction_calculator.init_tables();
    correction_calculator
}

pub struct CorrectionCalculator {
    pub gf_log: Vec<u32>,
    pub gf_exp: Vec<u32>
}

impl CorrectionCalculator {
    // for version 1 and error correction L
    pub fn _add_error_correction(_data: Vec<String>) {
        // Total number of codewords: 26
        // Number of error correction codewords: 7
        // Number of error correction blocks: 1
        // Error correction code per block: (26,19,2)
    }

    pub fn init_tables(&mut self) {
        let mut gf_exp: Vec<u32> = (0..512).collect();
        let mut gf_log: Vec<u32> = (0..256).collect();

        let mut count_helper = 0;
        let mut x = 1;
        for i in gf_exp.iter_mut() {
            *i = x;
            if count_helper < 255 {
                gf_log[x as usize] = count_helper;
                count_helper += 1;
            }
            x = gf_mult_no_lut(x, 2, PRIM);
        }

        for i in 255..512 {
            gf_exp[i] = gf_exp[i-255]
        }
        self.gf_exp = gf_exp;
        self.gf_log = gf_log;
    }

}

fn gf_mult_no_lut(x: u32, y:u32, prim: u32) -> u32 {

    fn cl_mult(x: u32, y: u32) -> u32 {
        let mut z = 0;
        let mut i = 0;
        while (y >> i) > 0 {
            if (y & (1 << i)) == 0 {
                z ^= (x << i)*2;
            }
            i += 1
        }
        z
    }

    // compute position of the most significant bit
    fn bit_length(n: u32) -> u32 {
        let mut bits = 0;
        for i in 0..(n.count_ones()+n.count_zeros()) {
            if (n >> i & 1) == 1 {
                bits = i;
            }
        }

        bits+1
    }

    // carry-less long division, returns remainder
    fn cl_div(mut dividend: u32, divisor: u32) -> u32 {
        let dl1 = bit_length(dividend);
        let dl2 = bit_length(divisor);

        if dl1 < dl2 {
            return dividend;
        }

        for i in (0..dl1-dl2+1).rev() {
            if (dividend & (1 << (i+dl2-1))) != 0 {
                dividend ^= divisor << i;
            }
        }
        dividend
    }

    let mut result = cl_mult(x, y);

    // modular reduction
    if prim > 0 {
        result = cl_div(result, prim);
    }
    result
}