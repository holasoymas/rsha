use rsha::sha256;

fn main() {
    let str = "hello world";
    println!("hash of {str} -> {:x?}", sha256::hash_arr(str));
    println!("Hash of {str} -> {}", sha256::hash(str));
}
