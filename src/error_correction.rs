// Source: https://en.wikiversity.org/wiki/Reed%E2%80%93Solomon_codes_for_coders#RS_encoding

use std::cmp;

const PRIM: i32 = 0x11d;

pub fn build_correction_calculator() -> CorrectionCalculator {
    let mut correction_calculator = CorrectionCalculator {
        gf_log: Vec::new(),
        gf_exp: Vec::new()
    };
    correction_calculator.init_tables();
    correction_calculator
}

#[derive(Clone)]
pub struct CorrectionCalculator {
    pub gf_log: Vec<i32>,
    pub gf_exp: Vec<i32>
}

impl CorrectionCalculator {
    pub fn init_tables(&mut self) {
        let mut gf_exp: Vec<i32> = (0..512).collect();
        let mut gf_log: Vec<i32> = (0..256).collect();

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

    // for version 1 and error correction L
    pub fn _add_error_correction(_data: Vec<String>) {
        // Total number of codewords: 26
        // Number of error correction codewords: 7
        // Number of error correction blocks: 1
        // Error correction code per block: (26,19,2)
    }

    fn gf_mul(self, x: i32, y: i32) -> i32 {
        if x == 0 || y == 0 {
            return 0;
        }
        self.gf_exp[(self.gf_log[x as usize] + self.gf_log[y as usize]) as usize]
    }

    fn gf_div(self, x: i32, y: i32) -> i32 {
        if y == 0 {
            panic!("ZeroDivision");
        } 
        if x == 0 {
            return 0;
        }
        self.gf_exp[((self.gf_log[x as usize]+255 - self.gf_log[y as usize]) % 255) as usize]
    }

    fn gf_pow(self, x: i32, power: i32) -> i32 {
        self.gf_exp[(self.gf_log[x as usize] * power) as usize]
    }

    fn gf_inverse(self, x: i32) -> i32 {
        self.gf_exp[(255 - self.gf_log[x as usize]) as usize]
    }

    fn gf_poly_scale(self, p: Vec<i32>, x: i32) -> Vec<i32> {
        let mut r: Vec<i32> = (0..p.len() as i32).collect(); 
        for i in 0..p.len() {
            r[i] = self.clone().gf_mul(p[i], x); 
        }
        r
    }

    fn gf_poly_add(self, p: Vec<i32>, q: Vec<i32>) -> Vec<i32> {
        let mut r: Vec<i32> = (0..cmp::max(p.len(), q.len()) as i32).collect(); 
        let len_tmp = r.len();
        for i in 0..p.len() {
            r[i+len_tmp-p.len()] = p[i];
        }
        for i in 0..q.len() {
            r[i+len_tmp-q.len()] ^= q[i];
        }
        r
    }

    fn gf_poly_mul(self, p: Vec<i32>, q: Vec<i32>) -> Vec<i32> {
        let mut r: Vec<i32> = (0..cmp::max(p.len(), q.len()-1) as i32).collect(); 

        for j in 0..q.len() {
            for i in 0..p.len() {
                r[i+j] ^= self.clone().gf_mul(p[i], q[j]);
            }
        }
        r
    }

    fn gf_poly_eval(self, poly: Vec<i32>, x: i32) -> i32 {
        let mut y = poly[0];
        for i in 1..poly.len() {
            y = self.clone().gf_mul(y, x) ^ poly[i];
        }
        y
    }

    fn gf_poly_div(self, dividend: Vec<i32>, divisor: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
        // not sure if this actually copies, need to investigate if something goes wrong!
        let mut msg_out = dividend.to_vec();
        for i in 0..dividend.len() {
            let coef = msg_out[1];
            if coef != 0 {
                for j in 1..divisor.len() {
                    if divisor[j] != 0 {
                        msg_out[i+j] ^= self.clone().gf_mul(divisor[j], coef);
                    }
                }
            }
        }
        let separator = -((divisor.len()-1) as i32);
        return (msg_out[(0..separator as usize)].to_vec(), msg_out[separator as usize..msg_out.len()].to_vec())
    }

    fn rs_generatory_poly(self, n_symbols: i32) {
        let mut g_poly: Vec<i32> = (0..n_symbols).collect();
        for i in 0..n_symbols {
            g_poly = self.clone().gf_poly_mul(g_poly, [1, self.clone().gf_pow(2, i)].to_vec());
        }
    }

}

fn gf_mult_no_lut(x: i32, y:i32, prim: i32) -> i32 {

    fn cl_mult(x: i32, y: i32) -> i32 {
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
    fn bit_length(n: i32) -> i32 {
        let mut bits = 0;
        for i in 0..(n.count_ones()+n.count_zeros()) {
            if (n >> i & 1) == 1 {
                bits = i;
            }
        }

        (bits+1) as i32
    }

    // carry-less long division, returns remainder
    fn cl_div(mut dividend: i32, divisor: i32) -> i32 {
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