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
struct BinTexReceiver {
    vis: syn::Visibility,
    ident: syn::Ident,
    generics: syn::Generics,
    data: ast::Data<BinTexVariantReceiver, BinTexFieldReceiver>,

    bit_width: Option<Num>,
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

        let mut bitfields = String::new();
        let bit_width = self.bit_width.as_ref().unwrap().0.base10_parse::<u8>()?;
        bitfields.push_str("\\begin{figure}\n");
        bitfields.push_str(&format!("\\begin{{bytefield}}{{{}}}\n", bit_width));
        bitfields.push_str(&format!("\\bitheader{{0-{}}} \\\\\n", bit_width - 1));

        let mut total_bits = 0;
        for field in fields {
            let ident = field
                .ident
                .as_ref()
                .unwrap()
                .to_string()
                .replace("_", "\\_");
            let bits = field.bits.as_ref().unwrap().0.base10_parse::<u8>()?;
            bitfields.push_str(&format!("\\bitbox{{{}}}{{{}}}", bits, ident));
            total_bits += bits;
            if (total_bits % bit_width) == 0 {
                bitfields.push_str(" \\\\\n");
            } else {
                bitfields.push_str(" & ");
            }
        }

        bitfields.push_str("\\end{bytefield}\n");
        bitfields.push_str(&format!("\\caption{{{}}}\n", ident.to_string()));
        bitfields.push_str("\\end{figure}");

        Ok(quote! {impl BinTexOutput for #ident {
            fn latex_output() -> String {
                #bitfields.to_string()
            }
        }})
    }
}

#[derive(Debug, FromVariant)]
#[darling(attributes(deku))]
struct BinTexVariantReceiver {
    ident: syn::Ident,
    fields: ast::Fields<BinTexFieldReceiver>,
    discriminant: Option<syn::Expr>,
}

/// Receiver for the field-level attributes inside a struct/enum variant
#[derive(Debug, FromField)]
#[darling(attributes(deku))]
struct BinTexFieldReceiver {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    /// field bit size
    #[darling(default)]
    bits: Option<Num>,
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
