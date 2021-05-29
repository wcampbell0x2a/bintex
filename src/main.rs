use bintex::{BinTex, BinTexOutput};
use std::fs::File;
use std::io::prelude::*;

fn main() {
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

    let mut file = File::create("TestingBytefield.tex").unwrap();
    file.write_all(Ipv6::latex_output().as_bytes());
}
