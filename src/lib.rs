#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#![warn(
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    clippy::cargo,
)]

extern crate proc_macro;
use proc_macro::TokenStream;
use std::ops::Deref;
use syn::{parse_macro_input, LitStr, LitInt, LitBool, Expr, Error};
use quote::{quote, ToTokens};

use syn::parse::{Parse, ParseStream};
use syn::token::Comma;

enum AcceptedLit {
    Int(LitInt),
    Str(LitStr),
    Bool(LitBool)
}

impl ToTokens for AcceptedLit {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Int(lit) => tokens.extend(quote!(#lit)),
            Self::Str(lit) => tokens.extend(quote!(#lit)),
            Self::Bool(lit) => tokens.extend(quote!(#lit))
        }
    }
}

impl Parse for AcceptedLit {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitInt) {
            input.parse().map(AcceptedLit::Int)
        } else if lookahead.peek(LitStr) {
            input.parse().map(AcceptedLit::Str)
        } else if lookahead.peek(LitBool) {
            input.parse().map(AcceptedLit::Bool)
        } else {
            Err(lookahead.error())
        }
    }
}

enum ArgType {
    Literal(AcceptedLit),
    Expr(Expr),
    Filler
}

impl Parse for ArgType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(accepted_lit) = input.parse::<AcceptedLit>() {
            Ok(Self::Literal(accepted_lit))
        } else {
            Ok(Self::Expr(input.parse().map_err(|_| {
                Error::new(
                    input.span(),
                    "Invalid argument, must be a literal or macro invocation. \
                     (Same constraints as `concat!`)"
                )
            })?))
        }
    }
}

impl ToTokens for ArgType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Literal(val) => tokens.extend(quote!(#val)),
            Self::Expr(val) => tokens.extend(quote!(#val)),
            Self::Filler => {}
        }
    }
}

#[derive(Debug)]
struct Template {
    slots: Vec<String>
}

impl Deref for Template {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.slots
    }
}

impl Parse for Template {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lit: LitStr = input.parse()
            .map_err(|_| Error::new(
                input.span(),
                "Requires at least one argument. \
                 Please provide the necessary format string and arguments."
            ))?;
        Ok(Self { slots: lit.value().split("{}").map(String::from).collect() })
    }
}

struct Format {
    template: Template,
    arguments: Vec<ArgType>
}

impl Parse for Format {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let template: Template = input.parse()?;
        let mut arguments = Vec::with_capacity(template.len());

        while input.parse::<Comma>().is_ok() {
            arguments.push(input.parse::<ArgType>()?);
        };

        arguments.push(ArgType::Filler);

        if arguments.len() != template.len() {
            return Err(Error::new(
                input.span(),
                format!(
                    "Wrong number of arguments, expected: {} but found {}",
                    template.len() - 1, arguments.len() - 1
                )
            ));
        }

        Ok(Self {
            template,
            arguments
        })
    }
}

impl Format {
    pub fn tokens(self) -> proc_macro2::TokenStream {
        let Template { slots } = self.template;
        let arguments = self.arguments;
        let tokens: Vec<_> = slots.into_iter().zip(arguments)
            .map(|(start, arg)| {
                quote! { #start, #arg }
            })
            .collect();

        quote!( concat!( #(#tokens),* ))
    }
}

/// `concat!` that feels like `format!`
///
/// <br>
///
/// String formatting with no runtime overhead, granted only with determined inputs at compile time.
///
/// ```
/// use static_format::const_format;
///
/// let my_str = const_format!("hello world {}", "...");
///
/// macro_rules! some_macro {
///     () => {"I am here"}
/// }
///
/// let my_str = const_format!("hello world {} all formatted", some_macro!());
/// ```
#[proc_macro]
pub fn const_format(input: TokenStream) -> TokenStream {
    let format = parse_macro_input!(input as Format);
    format.tokens().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream;

    #[test]
    fn test_basic_format() {
        let input: TokenStream = quote! {
            "hello {} world {}", "Rust", "Macro"
        };
        let parsed_input = syn::parse2::<Format>(input).unwrap();
        let generated = parsed_input.tokens();

        assert_eq!(generated.to_string(), "concat ! (\"hello \" , \"Rust\" , \" world \" , \"Macro\" , \"\" ,)");
    }

    #[test]
    fn test_basic_format_with_macro() {
        let input: TokenStream = quote! {
            "hello {} world {}, {} hello", "Rust", "Macro", a_macro!()
        };
        let parsed_input = syn::parse2::<Format>(input).unwrap();
        let generated = parsed_input.tokens();

        assert_eq!(
            generated.to_string(), "concat ! (\"hello \" , \"Rust\" , \" world \" , \"Macro\" , \", \" , a_macro ! () , \" hello\" ,)"
        );
    }
}