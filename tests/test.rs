#[test]
fn all_bits() {
    use bintex::{BinTex, BinTexOutput};

    #[allow(dead_code)]
    #[derive(BinTex)]
    #[bintex(bit_width = "8")]
    struct Testing {
        #[deku(bits = "4")]
        a: u8,
        #[deku(bits = "4")]
        b: u8,
        #[deku(bits = "8")]
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

#[test]
fn use_bits_from_type() {
    use bintex::{BinTex, BinTexOutput};

    #[allow(dead_code)]
    #[derive(BinTex)]
    #[bintex(bit_width = "8")]
    struct Testing {
        #[deku(bits = "4")]
        a: u8,
        #[deku(bits = "4")]
        b: u8,
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
