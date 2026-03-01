mod mc;
mod search;
fn main() {
    mc::run();
    search::run();
}

// fn fake_biomes(seed: i64) -> bool {
//     seed % 7 == 0
// }

// fn main() {
//     println!("Hello, world!");
//     for seed in 0..1000 {
//         if fake_biomes(seed) {
//             println!("found seed: {}", seed)
//         }
//     }
// }
