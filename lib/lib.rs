pub mod sqmul {
    pub fn square_mult(x: u32, pow: String, modulus: u32) -> u32 {
        let mut r = 1; 
        let t = pow.len();
        for i in (0..t).rev() {
            r *= r;
            r %= modulus;

            if pow.chars().nth(t-i-1).unwrap() == '1' {
                r *= x;
                r %= modulus;
            }
        }

        return r;
    }

    pub fn eea(mut r0: i32, mut r1: i32) -> (i32, i32, i32) {
        let mut prev_prev_r = r0;
        let mut prev_r = r1;
        let mut r = 0;

        let mut prev_prev_s = 1;
        let mut prev_s = 0;
        let mut s = 0;

        let mut prev_prev_t = 0;
        let mut prev_t = 1;
        let mut t = 0;

        let mut prev_prev_q = 0;
        let mut prev_q = 0;
        
        r = prev_prev_r % prev_r;
        prev_q = (prev_prev_r - r) / prev_r;
        s = prev_prev_s - (prev_q * prev_s);
        t = prev_prev_t - (prev_q * prev_t);

        while r != 0 {
            prev_prev_t = prev_t;
            prev_prev_s = prev_s;
            prev_prev_r = prev_r;

            prev_t = t;
            prev_s = s;
            prev_r = r;

            r = prev_prev_r % prev_r;
            prev_q = (prev_prev_r - r) / prev_r;
            s = prev_prev_s - (prev_q * prev_s);
            t = prev_prev_t - (prev_q * prev_t);
        }

        return (prev_r, prev_s, prev_t);
    }
}
