use bintex::prelude::*;
use deku::prelude::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    #[derive(BinTex, DekuRead, DekuWrite)]
    #[bintex(bit_width = 4)]
    struct Version {
        #[deku(bits = "4")]
        version: u8,
    }

    #[derive(BinTex, DekuRead, DekuWrite)]
    #[bintex(bit_width = 32)]
    struct Ipv6 {
        v: Version,
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

    let mut file = File::create("TestingBytefield.tex").unwrap();
    file.write_all(Version::latex_output().as_bytes());
    file.write_all(Ipv6::latex_output().as_bytes());
}
