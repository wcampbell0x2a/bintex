# bintex

(Work in Progress) Create latex [bytefield](https://www.ctan.org/pkg/bytefield) diagram with the use of rust proc-macros.

# Example

Run `>cargo run && pdflatex sample.tex` to create the following illustration from code:
```rust
#[derive(BinTex)]
#[bintex(bit_width = 32)]
struct Ipv6 {
    #[bintex(bits = "4")]
    version: u8,
    #[bintex(bits = "6")]
    ds: u8,
    #[bintex(bits = "2")]
    ecn: u8,
    #[bintex(bits = "20")]
    label: u32,
    #[bintex(bits = "16")]
    length: u16,
    #[bintex(bits = "8")]
    next_header: u8,
    #[bintex(bits = "8")]
    hop_limit: u8,
    #[bintex(bits = "32")]
    src: u32,
    #[bintex(bits = "32")]
    dst: u32,
}
```
![Result](/media/ipv6.png)
