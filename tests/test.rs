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

    let expected = r#"\begin{figure}
\begin{bytefield}{8}
\bitheader{0-7} \\
\bitbox{4}{a} & \bitbox{4}{b} \\
\bitbox{8}{c} \\
\end{bytefield}
\caption{Testing}
\end{figure}"#;
    assert_eq!(Testing::latex_output(), expected);
}
