/*!
A documentation-only module for #\[bintex\] attributes

All attributes are able to be used from only bintex, but can be also used with #\[deku\].

For example, bintex only.
```rust
use bintex::prelude::*;

#[allow(dead_code)]
#[derive(BinTex)]
#[bintex(bit_width = "8", bitheader = "0, 7")]
struct Testing {
    #[bintex(bits = "4")]
    a: u8,
    #[bintex(bits = "4")]
    b: u8,
    #[bintex(bits = "8")]
    c: u8,
}
```

For example, both deku and bintex:
```rust
use deku::prelude::*;
use bintex::prelude::*;

#[allow(dead_code)]
#[derive(BinTex, DekuRead, DekuWrite)]
#[bintex(bit_width = "8", bitheader = "0, 7")]
struct Testing {
    #[deku(bits = "4")]
    a: u8,
    #[deku(bits = "4")]
    #[bintex(unused)]
    b: u8,
    #[deku(bits = "8")]
    c: u8,
}
```
*/
