pub mod prelude;
pub mod attributes;

pub use bintex_derive::*;

pub trait BinTexOutput {
    fn latex_output() -> String;
}
