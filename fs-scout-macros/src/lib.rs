//! This crate is an experiment! It can't guarantee correctness in all usecases.

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use regex::Regex;
use std::path::{Component, Components, Path};
use syn::{parse::Parse, parse_macro_input, Error, LitStr};

#[proc_macro]
pub fn exists(input: TokenStream) -> TokenStream {
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
pub fn valid(input: TokenStream) -> TokenStream {
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

fn validate_path<P: AsRef<Path> + Copy>(path: P, span: Span) -> syn::Result<()> {
    let path = path.as_ref();

    if path.is_absolute() {
        try_absolute(path, span)
    } else {
        let abs_current_dir = std::env::current_dir().expect("current dir should be valid");
        let abs_path = abs_current_dir.join(path);
        try_absolute(&abs_path, span)
    }
}

fn try_absolute(path: &Path, span: Span) -> syn::Result<()> {
    let mut components = path.components();
    match components.next() {
        None => return Err(Error::new(span, "empty path")),
        Some(comp) => match comp {
            Component::Prefix(_) => {
                if let Some(comp) = components.next() {
                    match comp {
                        Component::RootDir => {
                            return try_absolute_components(components, path, span);
                        }
                        _ => (),
                    }
                }
            }
            Component::RootDir => {
                return try_absolute_components(components, path, span);
            }
            _ => (),
        },
    }

    fn try_absolute_components(components: Components, path: &Path, span: Span) -> syn::Result<()> {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Move {
            Descend,
            Ascend,
        }

        let mut move_history = vec![];
        for component in components {
            match component {
                Component::Normal(name) => {
                    check_name(name.to_str().expect("path should be valid UTF-8"), span)?;
                    move_history.push(Move::Descend);
                }
                Component::ParentDir => move_history.push(Move::Ascend),
                _ => (),
            }
        }
        if let Some(i) = move_history.iter().rev().position(|&m| m == Move::Ascend) {
            let n = move_history.len() - i;
            let depth = move_history.iter().take(n).fold(0, |acc, &m| match m {
                Move::Descend => acc + 1,
                Move::Ascend => acc - 1,
            });

            if depth < 0 {
                return Err(Error::new(
                    span,
                    format!(
                        "the path \"{}\" refers to an item outside the root",
                        path.display()
                    ),
                ));
            }
        }
        Ok(())
    }

    Ok(())
}

fn check_name(name: &str, span: Span) -> syn::Result<()> {
    let invalid_sequences =
        Regex::new(r#"[<>:"/\\|?*\x00-\x1F]|^(?i:CON|PRN|AUX|NUL|COM[0-9]|LPT[0-9])(?:\..+)?$"#)
            .unwrap();
    if invalid_sequences.is_match(name) {
        Err(Error::new(
            span,
            format!(
                "invalid character sequence: \"{}\"",
                invalid_sequences.find(name).unwrap().as_str()
            ),
        ))
    } else {
        let invalid_trailing = Regex::new(r".*[. ]$").unwrap();
        if invalid_trailing.is_match(name) {
            Err(Error::new(
                span,
                format!(
                    "invalid character in trailing position: '{}'",
                    invalid_trailing
                        .find(name)
                        .unwrap()
                        .as_str()
                        .chars()
                        .last()
                        .expect("should be non-empty")
                ),
            ))
        } else {
            Ok(())
        }
    }
}
