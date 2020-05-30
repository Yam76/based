[![Documentation](https://docs.rs/based/badge.svg)](https://docs.rs/based)
[![Crate](https://img.shields.io/crates/v/based.svg)](https://crates.io/crates/based)

# based

`based` provides support for custom numerical bases. `based` does not support multi-character digits.

# Example

```rust
use based::{Base, NumeralSystem};

let base16: Base = "0123456789abcdef".parse().unwrap();
let val: usize = base16.decode("10").unwrap();
assert_eq!(val, 16);
assert_eq!(base16.encode(16 as usize).unwrap(), "10")
```