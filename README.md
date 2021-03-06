# bintex

(Work in Progress) Create LaTeX [bytefield](https://www.ctan.org/pkg/bytefield) diagrams with the
use of rust proc-macros and the [deku](https://github.com/sharksforarms/deku) library.

# docs
run `cargo doc --open`

See `bintex::attribute` for Attribute details and examples.

# example

Run `>cargo run && pdflatex sample.tex` to create the following illustration from code:

```rust
use bintex::{BinTex, BinTexOutput};
use deku::prelude::*;

#[derive(BinTex)]
#[bintex(bit_width = 32)]
struct Ipv6 {
    #[deku(bits = "4")]
    version: u8,
    #[deku(bits = "6")]
    ds: u8,
    #[deku(bits = "2")]
    ecn: u8,
    #[deku(bits = "20")]
    label: u32,
    length: u16,
    next_header: u8,
    hop_limit: u8,
    src: u32,
    dst: u32,
}
```
![Result](/media/ipv6.png)
