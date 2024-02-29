//! This crate is an experiment! It can't guarantee correctness in all usecases.

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::path::{Component, Components, Path};
use syn::{parse::Parse, parse_macro_input, Error, LitStr};

#[proc_macro]
pub fn exists(input: TokenStream) -> TokenStream {
    let ExistsData(s) = parse_macro_input!(input as ExistsData);
    quote! {
        #s
    }
    .into()
}

struct ExistsData(String);

impl Parse for ExistsData {
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

            Ok(ExistsData(path_string))
        } else {
            Err(Error::new(
                input.span(),
                "invalid input, expected string literal",
            ))
        }
    }
}

#[proc_macro]
pub fn valid_file(input: TokenStream) -> TokenStream {
    let ValidFileData(s) = parse_macro_input!(input as ValidFileData);
    quote! {
        #s
    }
    .into()
}

struct ValidFileData(String);

impl Parse for ValidFileData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            let lit = input.parse::<LitStr>()?;
            let path_string = lit.value();
            let path = Path::new(&path_string);
            let span = lit.span();

            validate_file_path(path, span)?;

            Ok(ValidFileData(path_string))
        } else {
            Err(Error::new(
                input.span(),
                "invalid input, expected string literal",
            ))
        }
    }
}

fn validate_file_path<P: AsRef<Path> + Copy>(path: P, span: Span) -> syn::Result<()> {
    let path = path.as_ref();

    if let None = path.extension() {
        return Err(Error::new(span, "this path belongs to a directory"));
    }

    if path.is_absolute() {
        try_absolute_file(path, span)
    } else {
        let abs_current_dir = std::env::current_dir().expect("current dir should be valid");
        let abs_path = abs_current_dir.join(path);
        try_absolute_file(&abs_path, span)
    }
}

fn try_absolute_file(path: &Path, span: Span) -> syn::Result<()> {
    match path.parent() {
        Some(parent) => {
            if let Ok(true) = parent.try_exists() {
            } else {
                return Err(Error::new(
                    span,
                    format!("parent path {} doesn't exist", parent.display()),
                ));
            }
        }
        None => return Ok(()),
    }

    match path.components().last() {
        Some(last) => match last {
            Component::Normal(name) => {
                #[cfg(feature = "win")]
                {
                    return check_name(name.to_str().expect("path should be valid UTF-8"), span);
                }
                #[allow(unreachable_code)]
                Ok(())
            }
            _ => Err(Error::new(span, "wrong component type")),
        },
        None => Err(Error::new(span, "empty path")),
    }
}

#[proc_macro]
pub fn valid_dir(input: TokenStream) -> TokenStream {
    let ValidDirData(s) = parse_macro_input!(input as ValidDirData);
    quote! {
        #s
    }
    .into()
}

struct ValidDirData(String);

impl Parse for ValidDirData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            let lit = input.parse::<LitStr>()?;
            let path_string = lit.value();
            let path = Path::new(&path_string);
            let span = lit.span();

            validate_dir_path(path, span)?;

            Ok(ValidDirData(path_string))
        } else {
            Err(Error::new(
                input.span(),
                "invalid input, expected string literal",
            ))
        }
    }
}

fn validate_dir_path<P: AsRef<Path> + Copy>(path: P, span: Span) -> syn::Result<()> {
    let path = path.as_ref();

    if let Some(_) = path.extension() {
        return Err(Error::new(span, "this path belongs to a file"));
    }

    if path.is_absolute() {
        try_absolute_dir(path, span)
    } else {
        let abs_current_dir = std::env::current_dir().expect("current dir should be valid");
        let abs_path = abs_current_dir.join(path);
        try_absolute_dir(&abs_path, span)
    }
}

fn try_absolute_dir(path: &Path, span: Span) -> syn::Result<()> {
    match path.parent() {
        Some(parent) => {
            if let Ok(true) = parent.try_exists() {
            } else {
                return Err(Error::new(
                    span,
                    format!("parent path {} doesn't exist", parent.display()),
                ));
            }
        }
        None => return Ok(()),
    }

    match path.components().last() {
        Some(last) => match last {
            Component::Normal(name) => {
                #[cfg(feature = "win")]
                {
                    return check_name(name.to_str().expect("path should be valid UTF-8"), span);
                }
                #[allow(unreachable_code)]
                Ok(())
            }
            _ => Err(Error::new(span, "wrong component type")),
        },
        None => Err(Error::new(span, "empty path")),
    }
}

#[proc_macro]
pub fn valid_dir_all(input: TokenStream) -> TokenStream {
    let ValidDirAllData(s) = parse_macro_input!(input as ValidDirAllData);
    quote! {
        #s
    }
    .into()
}

struct ValidDirAllData(String);

impl Parse for ValidDirAllData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            let lit = input.parse::<LitStr>()?;
            let path_string = lit.value();
            let path = Path::new(&path_string);
            let span = lit.span();

            validate_dir_all_path(path, span)?;

            Ok(ValidDirAllData(path_string))
        } else {
            Err(Error::new(
                input.span(),
                "invalid input, expected string literal",
            ))
        }
    }
}

fn validate_dir_all_path<P: AsRef<Path> + Copy>(path: P, span: Span) -> syn::Result<()> {
    let path = path.as_ref();

    if let Some(_) = path.extension() {
        return Err(Error::new(span, "this path belongs to a file"));
    }

    if path.is_absolute() {
        try_absolute_dir_all(path, span)
    } else {
        let abs_current_dir = std::env::current_dir().expect("current dir should be valid");
        let abs_path = abs_current_dir.join(path);
        try_absolute_dir_all(&abs_path, span)
    }
}

fn try_absolute_dir_all(path: &Path, span: Span) -> syn::Result<()> {
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
                Component::Normal(_name) => {
                    #[cfg(feature = "win")]
                    {
                        check_name(_name.to_str().expect("path should be valid UTF-8"), span)?;
                    }
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

#[cfg(feature = "win")]
fn check_name(name: &str, span: Span) -> syn::Result<()> {
    let invalid_trailing = regex::Regex::new(r".*[. ]$").unwrap();
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
        let invalid_sequences = regex::Regex::new(
            r#"[<>:"/\\|?*\x00-\x1F]|^(?i:CON|PRN|AUX|NUL|COM[0-9]|LPT[0-9])(?:\..+)?$"#,
        )
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
            Ok(())
        }
    }
}
