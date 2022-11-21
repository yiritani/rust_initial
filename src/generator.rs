use rand::Rng;
pub fn generate_num() -> String {
    let mut rng = rand::thread_rng();
    let num: u32 = rng.gen();
    num.to_string()
}
