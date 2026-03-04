pub struct LayerRNG {
    layer_seed: i64,
    cell_seed: i64,
}

impl LayerRNG {
    pub fn new(world_seed: i64, salt: i64) -> Self {
        let layer_seed = layer_seed(world_seed, salt);
        Self {
            layer_seed,
            cell_seed: 0,
        }
    }

    pub fn set_position(&mut self, x: i64, z: i64) {
        let mut s = self.layer_seed;

        s = mix(s, x);
        s = mix(s, z);
        s = mix(s, x);
        s = mix(s, z);

        self.cell_seed = s;
    }

    pub fn next_int(&mut self, bound: i32) -> i32 {
        let b = bound as i64;

        let mut r = (self.cell_seed >> 24) % b;
        if r < 0 {
            r += b;
        }

        self.cell_seed = mix(self.cell_seed, self.layer_seed);

        r as i32
    }
}

fn mix(seed: i64, value: i64) -> i64 {
    let mut s = seed;
    s = s.wrapping_mul(
        s.wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407),
    );
    s = s.wrapping_add(value);
    s
}

fn layer_seed(world_seed: i64, salt: i64) -> i64 {
    let mut s = mix(salt, salt);
    s = mix(s, salt);
    s = mix(s, salt);

    let mut w = mix(world_seed, s);
    w = mix(w, s);
    w = mix(w, s);

    w
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic() {
        let mut rng1 = LayerRng::new(12345, 1);
        let mut rng2 = LayerRng::new(12345, 1);

        rng1.set_position(10, 20);
        rng2.set_position(10, 20);

        for _ in 0..10 {
            assert_eq!(rng1.next_int(100), rng2.next_int(100));
        }
    }
}
