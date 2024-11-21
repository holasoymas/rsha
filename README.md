# rsha

A simple to use hashing library in rust

# install

`cargo add rsha`

# Examples

```
use rsha::sha256;

let str = "शा तुम कितनी सुन्दर हो";
let hash = sha256::hash(str);
println("{str} -> {hash}");
```
