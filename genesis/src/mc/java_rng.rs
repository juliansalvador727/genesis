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
        ((self.seed) >> (48 - bits)) as u32
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
        let high = self.next(32) as i64; //this is stupid but java
        let low = self.next(32) as i64;
        (high << 32).wrapping_add(low)
    }
}

#[cfg(test)]
mod tests {
    use super::JavaRNG;

    // 1) determinism: same seed => same sequence
    #[test]
    fn same_seed_same_sequence() {
        let mut a = JavaRNG::new(999);
        let mut b = JavaRNG::new(999);

        for _ in 0..1000 {
            assert_eq!(a.next_int(1_000_000), b.next_int(1_000_000));
        }
    }

    // 2) different seeds should diverge quickly (not a proof, but a good sanity check)
    #[test]
    fn different_seeds_diverge() {
        let mut a = JavaRNG::new(999);
        let mut b = JavaRNG::new(1000);

        // extremely unlikely to be equal for all these draws if working
        let mut same = 0;
        for _ in 0..200 {
            if a.next_int(1_000_000) == b.next_int(1_000_000) {
                same += 1;
            }
        }
        assert!(same < 5);
    }

    // 3) bound=1 must always return 0
    #[test]
    fn next_int_bound_one_is_zero() {
        let mut r = JavaRNG::new(123);
        for _ in 0..10_000 {
            assert_eq!(r.next_int(1), 0);
        }
    }

    // 4) range property: always 0 <= value < bound
    #[test]
    fn next_int_in_range_many_bounds() {
        let mut r = JavaRNG::new(12345);
        let bounds = [
            2, 3, 4, 5, 7, 8, 10, 16, 31, 32, 33, 97, 100, 127, 128, 129, 1024,
        ];
        for &b in &bounds {
            for _ in 0..5000 {
                let x = r.next_int(b);
                assert!(0 <= x && x < b, "x={} not in [0, {})", x, b);
            }
        }
    }

    // 5) power-of-two bounds still in range (hits that fast path)
    #[test]
    fn next_int_power_of_two_bounds() {
        let mut r = JavaRNG::new(777);
        let bounds = [2, 4, 8, 16, 32, 64, 128, 1024, 1 << 20];
        for &b in &bounds {
            for _ in 0..5000 {
                let x = r.next_int(b);
                assert!(0 <= x && x < b);
            }
        }
    }

    // 6) non-power-of-two bounds (forces rejection path)
    #[test]
    fn next_int_rejection_bounds() {
        let mut r = JavaRNG::new(888);
        let bounds = [3, 5, 6, 7, 9, 10, 12, 31, 33, 1000, 1_000_003];
        for &b in &bounds {
            for _ in 0..5000 {
                let x = r.next_int(b);
                assert!(0 <= x && x < b);
            }
        }
    }

    // 7) set_seed resets sequence
    #[test]
    fn set_seed_resets_sequence() {
        let mut r = JavaRNG::new(1);
        let a1 = r.next_int(1_000_000);
        let a2 = r.next_int(1_000_000);

        r.set_seed(1);
        let b1 = r.next_int(1_000_000);
        let b2 = r.next_int(1_000_000);

        assert_eq!(a1, b1);
        assert_eq!(a2, b2);
    }

    // 8) negative and large seeds should work (just deterministic + in range)
    #[test]
    fn negative_seed_and_large_seed_work() {
        let mut a = JavaRNG::new(-1);
        let mut b = JavaRNG::new(i64::MAX);

        for _ in 0..10_000 {
            let x = a.next_int(1_000_000);
            let y = b.next_int(1_000_000);
            assert!(0 <= x && x < 1_000_000);
            assert!(0 <= y && y < 1_000_000);
        }
    }

    // 9) next_long changes state and is deterministic across identical RNGs
    #[test]
    fn next_long_deterministic() {
        let mut a = JavaRNG::new(424242);
        let mut b = JavaRNG::new(424242);

        for _ in 0..1000 {
            assert_eq!(a.next_long(), b.next_long());
        }
    }

    // 10) mixing next_int and next_long preserves determinism (state transitions identical)
    #[test]
    fn mixed_calls_deterministic() {
        let mut a = JavaRNG::new(2026);
        let mut b = JavaRNG::new(2026);

        for i in 0..2000 {
            if i % 3 == 0 {
                assert_eq!(a.next_long(), b.next_long());
            } else {
                assert_eq!(a.next_int(10_000), b.next_int(10_000));
            }
        }
    }
}
