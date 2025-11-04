use rand::Rng;

pub fn run() {
    for _ in 1..10 {
        let n = rand::rng().random_range(1..=6);
        println!("Würfel zeigt: {}", n);
    }
}
