pub mod attributes;
pub mod prelude;

pub use bintex_derive::*;

pub trait BinTexOutput {
    fn latex_output() -> String;
}
