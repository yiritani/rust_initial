mod generator;

pub fn print_random_number() {
    println!("Random number: {}", generator::generate_num());
}
