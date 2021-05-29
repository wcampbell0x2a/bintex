use bintex::{BinTex, BinTexOutput};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    #[derive(BinTex)]
    #[bintex(bit_width = "8")]
    struct Testing {
        #[bintex(bits = "4")]
        a: u8,
        #[bintex(bits = "4")]
        b: u8,
        #[bintex(bits = "8")]
        c: u8,
    }

    let one = Testing {
        a: 0b1010,
        b: 0b0101,
        c: 0b1111,
    };

    let mut file = File::create("TestingBytefield.tex").unwrap();
    file.write_all(one.latex_output().as_bytes());
}
