use hash::hash;

fn main() {
    let value = "Hello, world!";
    let hash_value = hash(value);
    println!("Hash of '{}': {}", value, hash_value);
}
