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

#[test]
fn readme() {
    use bintex::{BinTex, BinTexOutput};

    #[allow(dead_code)]
    #[derive(BinTex)]
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
        length: u16,
        next_header: u8,
        hop_limit: u8,
        src: u32,
        dst: u32,
    }

    let expected = r#"\begin{figure}
\begin{bytefield}{32}
\bitheader{0-31} \\
\bitbox{4}{version} & \bitbox{6}{ds} & \bitbox{2}{ecn} & \bitbox{20}{label} \\
\bitbox{16}{length} & \bitbox{8}{next\_header} & \bitbox{8}{hop\_limit} \\
\bitbox{32}{src} \\
\bitbox{32}{dst} \\
\end{bytefield}
\caption{Ipv6}
\end{figure}"#;
    assert_eq!(Ipv6::latex_output(), expected);
}

#[test]
fn bitheader() {
    use bintex::{BinTex, BinTexOutput};

    #[allow(dead_code)]
    #[derive(BinTex)]
    #[bintex(bit_width = "8", bitheader = "0, 7")]
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
\bitheader{0, 7} \\
\bitbox{4}{a} & \bitbox{4}{b} \\
\bitbox{8}{c} \\
\end{bytefield}
\caption{Testing}
\end{figure}"#;
    assert_eq!(Testing::latex_output(), expected);
}

#[test]
fn unused() {
    use bintex::{BinTex, BinTexOutput};

    #[allow(dead_code)]
    #[derive(BinTex)]
    #[bintex(bit_width = "8", bitheader = "0, 7")]
    struct Testing {
        #[deku(bits = "4")]
        a: u8,
        #[deku(bits = "4")]
        #[bintex(unused)]
        b: u8,
        #[deku(bits = "8")]
        c: u8,
    }

    let expected = r#"\begin{figure}
\begin{bytefield}{8}
\bitheader{0, 7} \\
\bitbox{4}{a} & \bitbox{4}[bgcolor=lightgray]{} \\
\bitbox{8}{c} \\
\end{bytefield}
\caption{Testing}
\end{figure}"#;
    assert_eq!(Testing::latex_output(), expected);
}
