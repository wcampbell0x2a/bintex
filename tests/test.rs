#[allow(dead_code)]
#[test]
fn it_works() {
    use bintex::{BinTex, BinTexOutput};

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

    let expected =
        r#"\begin{bytefield}{8}
\bitheader{0, 7} \\
\bitbox{4}{a} & \bitbox{4}{b} \\
\bitbox{8}{c} \\
\end{bytefield}"#;
    assert_eq!(one.latex_output(), expected);
}
