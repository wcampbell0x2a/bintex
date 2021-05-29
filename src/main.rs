use bintex::{BinTex, BinTexOutput};
use std::fs::File;
use std::io::prelude::*;
use deku::prelude::*;

fn main() {
    #[derive(BinTex, DekuRead, DekuWrite)]
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
        #[deku(bits = "16")]
        length: u16,
        #[deku(bits = "8")]
        next_header: u8,
        #[deku(bits = "8")]
        hop_limit: u8,
        #[deku(bits = "32")]
        src: u32,
        #[deku(bits = "32")]
        dst: u32,
    }

    let mut file = File::create("TestingBytefield.tex").unwrap();
    file.write_all(Ipv6::latex_output().as_bytes());
}
