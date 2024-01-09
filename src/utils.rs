use rand::Rng;

pub fn random_string() -> String {
    let mut rng = rand::thread_rng();
    let random: u64 = rng.gen();
    format!("{:x}", random)
}
