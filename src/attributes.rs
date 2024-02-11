/*!
A documentation-only module for #\[bintex\] attributes

All attributes are able to be used from only bintex, but can be also used with #\[deku\].

For example, bintex only.
```rust
use bintex::prelude::*;

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

# List of attributes

|     Attribute           | Scope     |   Library   | Description
|-------------------------|-----------|-------------|--------------------------------------------------
| [bits](#bits)           | field     | deku/bintex | Set the bit-size of the field
| [unused](#unused)       | field     | bintex      | Set background color to gray, supress field ident
| [bit_width](#bit_width) | top-level | bintex      | Set width of bits in LaTeX bytefield
| [bitheader](#bitheader) | top-level | bintex      | Set bitheader for LaTeX bytefield

# bits

See [deku::bits](https://docs.rs/deku/0.12.1/deku/attributes/index.html#bits)

# unused

Set background color to gray and remove the field name from the LaTeX output.

Example:
```rust
use deku::prelude::*;
use bintex::prelude::*;

#[derive(BinTex, DekuRead, DekuWrite)]
#[bintex(bit_width = "32", bitheader = "0, 4, 8, 12, 16, 20, 24, 28")]
struct UnusedStruct {
    Tag: u8,
    Value: u8,
    #[bintex(unused)]
    #[deku(bits = "4")]
    c: u8,
    #[deku(bits = "12")]
    Mask: u16,
    Key: u32,
}
```

[example output](https://github.com/wcampbell0x2a/bintex/blob/master/media/unused.png)

# bit_width

Set the bit_width of the total bytefield LaTeX figure.

# bitheader

Set a custom bitheader. Defaults to "0-(bit_width - 1)".
*/
