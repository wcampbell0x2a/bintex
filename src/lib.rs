pub use bintex_derive::*;

pub trait BinTexOutput {
    fn latex_output(&self) -> String;
}
