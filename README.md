# rsha

A simple to use hashing library in rust

# install

```bash
cargo add rsha
```

# Examples

```rust
use rsha::sha256;

fn main(){
let str = "知識は自由への鍵です。";
let hash = sha256::hash(str);
println("{str} -> {hash}");
}
```

This return a array of `[u32; 8]`

```rust
use rsha::sha256;

fn main(){
let str = "make me sha";
let hash = sha256::hash_arr(str);
println("{str} -> {hash}");
}
```
