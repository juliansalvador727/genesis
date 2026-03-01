// my implementation of random.java i.e. Java’s 48-bit LCG
// source: https://github.com/openjdk/jdk/blob/master/src/java.base/share/classes/java/util/Random.java#L357
const MULTIPLIER: u64 = 0x5DEECE66D;
const ADDEND: u64 = 0xB;
const MASK: u64 = (1 << 48) - 1;

pub struct JavaRNG {
    seed: u64,
}

impl JavaRNG {
    pub fn new(seed: i64) -> Self {
        let mut rng = JavaRNG { seed: 0 };
        rng.set_seed(seed);
        rng
    }
    pub fn set_seed(&mut self, seed: i64) {
        self.seed = ((seed as u64) ^ MULTIPLIER) & MASK;
    }
    fn next(&mut self, bits: u32) -> u32 {
        // use wrapping.mul() and wrapping_add() methods to mimic java
        // copying the java line
        // we set seed = seed * 0x5DEECE66DL + 0xBL) & ((1L << 48) - 1)
        // we output seed >>> (48 - bits)
        self.seed = (self.seed.wrapping_mul(MULTIPLIER).wrapping_add(ADDEND)) & MASK;
        ((self.seed) >> (48 - bits) as u32)
    }
    pub fn next_int(&mut self, bound: i32) -> i32 {
        assert!(bound > 0);

        let m = bound - 1;

        if (bound & m) == 0 {
            let r = self.next(31) as i64;
            return ((bound as i64 * r) >> 31) as i32;
        }

        loop {
            let u = self.next(31) as i32;
            let r = u % bound;

            if (u as i64) - (r as i64) + (m as i64) >= 0 {
                return r;
            }
        }
    }
    pub fn next_long(&mut self) -> i64 {
        let high = self.next(32) as u64;
        let low = self.next(32) as u64;
        ((high << 32) | low) as i64
    }
}
