extern crate proc_macro;

use fancy_regex::Regex;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::path::Path;
use syn::{parse::Parse, parse_macro_input, Error, LitStr};

#[proc_macro]
pub fn scout(input: TokenStream) -> TokenStream {
    let ScoutData(s) = parse_macro_input!(input as ScoutData);
    quote! {
        #s
    }
    .into()
}

struct ScoutData(String);

impl Parse for ScoutData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            let lit = input.parse::<LitStr>()?;
            let path_string = lit.value();
            let path = Path::new(&path_string);

            if let Ok(true) = path.try_exists() {
            } else {
                return Err(Error::new(lit.span(), "path doesn't exist"));
            }

            Ok(ScoutData(path_string))
        } else {
            Err(Error::new(
                input.span(),
                "invalid input, expected string literal",
            ))
        }
    }
}

#[proc_macro]
pub fn validate(input: TokenStream) -> TokenStream {
    let ValidateData(s) = parse_macro_input!(input as ValidateData);
    quote! {
        #s
    }
    .into()
}

struct ValidateData(String);

impl Parse for ValidateData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            let lit = input.parse::<LitStr>()?;
            let path_string = lit.value();
            let path = Path::new(&path_string);
            let span = lit.span();

            validate_path(path, span)?;

            Ok(ValidateData(path_string))
        } else {
            Err(Error::new(
                input.span(),
                "invalid input, expected string literal",
            ))
        }
    }
}

fn validate_path(path: &Path, span: Span) -> syn::Result<()> {
    let mut components = path.components();
    let invalid_sequence =
        Regex::new(r#"[<>:"/\\|?*\x00-\x1F]|^(?i:CON|PRN|AUX|NUL|COM[0-9]|LPT[0-9])(?:\..+)?$"#)
            .unwrap();
    match components.next() {
        None => return Err(Error::new(span, "empty path")),
        Some(comp) => match comp {
            std::path::Component::Normal(name) => check_name(
                &invalid_sequence,
                name.to_str().expect("path should be valid UTF-8"),
                span,
            )?,
            _ => (),
        },
    }
    Ok(())
}

fn check_name(regex: &Regex, name: &str, span: Span) -> syn::Result<()> {
    if regex.is_match(name).unwrap() {
        Err(Error::new(
            span,
            &format!(
                "invalid character sequence: {}",
                regex.find(name).unwrap().unwrap().as_str()
            ),
        ))
    } else {
        Ok(())
    }
}
