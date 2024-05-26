use huffman_coding::huffman_encode;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

fn main() {
    let text: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(1024)
        .map(|c| c as char)
        .collect();
    let encoded = huffman_encode(&text);
    println!("Encoded: {:?}", encoded);
}
