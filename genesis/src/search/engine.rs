pub fn run() {
    for seed in 0..1000 {
        if seed % 7 == 0 {
            println!("found seed {}", seed);
        }
    }
}
