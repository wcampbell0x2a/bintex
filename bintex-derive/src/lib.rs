use darling::{ast, FromDeriveInput, FromField, FromMeta, FromVariant, ToTokens};
use proc_macro2::TokenStream;
use quote::quote;

#[proc_macro_derive(BinTex, attributes(bintex, deku))]
pub fn proc_bintex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input);

    let receiver = match BinTexReceiver::from_derive_input(&input) {
        Ok(receiver) => receiver,
        Err(err) => return err.write_errors().into(),
    };

    receiver.emit().unwrap().into()
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(bintex), supports(struct_any, enum_any))]
#[allow(unused)]
struct BinTexReceiver {
    vis: syn::Visibility,
    ident: syn::Ident,
    generics: syn::Generics,
    data: ast::Data<BinTexVariantReceiver, BinTexFieldReceiver>,

    bit_width: Num,

    #[darling(default)]
    bitheader: Option<syn::LitStr>,
}

impl BinTexReceiver {
    fn emit(&self) -> Result<TokenStream, syn::Error> {
        match self.data {
            darling::ast::Data::Enum(_) => unreachable!("currently unsupported"),
            darling::ast::Data::Struct(_) => self.emit_struct(),
        }
    }

    fn emit_struct(&self) -> Result<TokenStream, syn::Error> {
        let (_, ty, _) = self.generics.split_for_impl();

        let ident = &self.ident;
        let ident = quote! { #ident #ty };

        let fields = self.data.as_ref().take_struct().unwrap();

        // parse required bit_with
        let bit_width = self.bit_width.0.base10_parse::<u8>()?;

        // either use 0-bitwidth - 1, or user defined string
        let bitheader = if let Some(bitheader) = self.bitheader.as_ref() {
            bitheader.value()
        } else {
            format!("0-{}", bit_width - 1)
        };

        // preamble
        let preamble = quote! {
                let mut input = String::new();
                input.push_str("\\begin{figure}\n");
                input.push_str(&format!("\\begin{{bytefield}}{{{}}}\n", #bit_width));
                input.push_str(&format!("\\bitheader{{{}}} \\\\\n", #bitheader));
        };

        // body
        let mut body = quote! {let mut total_bits: u8 = 0;};

        for field in fields {
            // create unused and ident (&str, String)
            let (unused, ident) = if field.unused.is_some() {
                ("[bgcolor=lightgray]", "".to_string())
            } else {
                (
                    "",
                    field
                        .ident
                        .as_ref()
                        .unwrap()
                        .to_string()
                        .replace('_', "\\_"),
                )
            };

            let field_ty = &field.ty;
            let token_bits = if let Some(ref bits) = field.bits {
                if let Ok(bits) = bits.0.base10_parse::<u8>() {
                    quote! { #bits }
                } else {
                    unreachable!()
                }
            } else {
                quote! { #field_ty::BITS }
            };
            body = quote! {
                #body

                input.push_str(&format!("\\bitbox{{{}}}{}{{{}}}", #token_bits, #unused, #ident));

                total_bits += #token_bits as u8;
                if (total_bits % #bit_width) == 0 {
                    input.push_str(" \\\\\n");
                } else {
                    input.push_str(" & ");
                }
            };
        }

        let self_ident_string = ident.to_string();
        let tokens = quote! {
        impl BinTexOutput for #ident {
            const BITS: usize = #bit_width as usize;
            fn latex_output() -> String {
                #preamble

                #body

                input.push_str("\\end{bytefield}\n");
                input.push_str(&format!("\\caption{{{}}}\n", #self_ident_string));
                input.push_str("\\end{figure}");
                input
            }
        }};

        Ok(tokens)
    }
}

#[derive(Debug, FromVariant)]
#[darling(attributes(deku))]
#[allow(unused)]
struct BinTexVariantReceiver {
    ident: syn::Ident,
    fields: ast::Fields<BinTexFieldReceiver>,
    discriminant: Option<syn::Expr>,
}

/// Receiver for the field-level attributes inside a struct/enum variant
#[derive(Debug, FromField)]
#[darling(attributes(deku, bintex))]
struct BinTexFieldReceiver {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    /// field bit size
    #[darling(default)]
    bits: Option<Num>,

    /// Use lightgray background color and suppress field name
    #[darling(default)]
    unused: Option<()>,
}

#[derive(Debug)]
struct Num(syn::LitInt);

impl Num {
    fn new(n: syn::LitInt) -> Self {
        Self(n)
    }
}

impl ToString for Num {
    fn to_string(&self) -> String {
        self.0.to_token_stream().to_string()
    }
}

impl ToTokens for Num {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

impl FromMeta for Num {
    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        (match *value {
            syn::Lit::Str(ref s) => Ok(Num::new(syn::LitInt::new(
                s.value()
                    .as_str()
                    .parse::<usize>()
                    .map_err(|_| darling::Error::unknown_value(&s.value()))?
                    .to_string()
                    .as_str(),
                s.span(),
            ))),
            syn::Lit::Int(ref s) => Ok(Num::new(s.clone())),
            _ => Err(darling::Error::unexpected_lit_type(value)),
        })
        .map_err(|e| e.with_span(value))
    }
}
