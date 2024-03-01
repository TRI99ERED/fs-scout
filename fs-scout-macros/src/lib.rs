//! This crate is an experiment! It can't guarantee correctness in all usecases.

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::{
    marker::PhantomData,
    path::{Component, Components, Path},
};
use syn::{parse::Parse, parse_macro_input, Error, LitStr};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Move {
    Descend,
    Ascend,
}

trait Matcher {
    fn try_match(path: &Path, span: Span) -> syn::Result<()>;

    fn try_parent_exists(path: &Path, span: Span) -> syn::Result<()> {
        match path.parent() {
            None => Ok(()),
            Some(parent) => match parent.try_exists() {
                Ok(true) => Ok(()),
                _ => Err(Error::new(
                    span,
                    format!("parent path \"{}\" doesn't exist", parent.display()),
                )),
            },
        }
    }

    fn try_absolute(path: &Path, span: Span) -> syn::Result<()> {
        match path.components().last() {
            None => return Err(Error::new(span, "empty path")),
            Some(last) => match last {
                Component::Normal(name) => {
                    #[cfg(feature = "win")]
                    {
                        check_name(name.to_str().expect("path should be valid UTF-8"), span)?;
                    }
                }
                _ => return Err(Error::new(span, "wrong component type")),
            },
        }

        let mut components = path.components();

        match components.next() {
            None => Err(Error::new(span, "empty path")),
            Some(comp) => match comp {
                Component::Prefix(_) => match components.next() {
                    None => Self::try_absolute_components(components, path, span),
                    Some(comp) => match comp {
                        Component::RootDir => Self::try_absolute_components(components, path, span),
                        _ => Err(Error::new(span, "wrong component type")),
                    },
                },
                Component::RootDir => Self::try_absolute_components(components, path, span),
                _ => Err(Error::new(span, "wrong component type")),
            },
        }
    }

    fn try_absolute_components(components: Components, path: &Path, span: Span) -> syn::Result<()> {
        let mut move_history = vec![];
        for component in components {
            match component {
                Component::Normal(name) => {
                    #[cfg(feature = "win")]
                    {
                        check_name(name.to_str().expect("path should be valid UTF-8"), span)?;
                    }
                    move_history.push(Move::Descend);
                }
                Component::ParentDir => move_history.push(Move::Ascend),
                _ => (),
            }
        }

        if let Some(i) = move_history.iter().rev().position(|&m| m == Move::Ascend) {
            let last_is_parent_dir = move_history.len() - i;
            let depth = move_history
                .iter()
                .take(last_is_parent_dir)
                .fold(0, |acc, &m| match m {
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
}

struct ScoutData<M: Matcher + ?Sized>(String, PhantomData<M>);

impl<M: Matcher> Parse for ScoutData<M> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            let lit = input.parse::<LitStr>()?;
            let path_string = lit.value();
            let path = Path::new(&path_string);
            let span = lit.span();

            M::try_match(path, span)?;

            Ok(Self(path_string, PhantomData))
        } else {
            Err(Error::new(
                input.span(),
                format!(
                    "invalid input, expected string literal; input: {}",
                    input.to_string()
                ),
            ))
        }
    }
}

struct ExistsMatcher;

impl Matcher for ExistsMatcher {
    fn try_match(path: &Path, span: Span) -> syn::Result<()> {
        if let Ok(true) = path.try_exists() {
            Ok(())
        } else {
            Err(Error::new(span, "path doesn't exist"))
        }
    }
}

#[proc_macro]
pub fn exists(input: TokenStream) -> TokenStream {
    let ScoutData(s, ..) = parse_macro_input!(input as ScoutData<ExistsMatcher>);
    quote! {
        #s
    }
    .into()
}

struct NotExistMatcher;

impl Matcher for NotExistMatcher {
    fn try_match(path: &Path, span: Span) -> syn::Result<()> {
        if let Ok(true) = path.try_exists() {
            Err(Error::new(span, "path already exists"))
        } else {
            Ok(())
        }
    }
}

#[proc_macro]
pub fn not_exists(input: TokenStream) -> TokenStream {
    let ScoutData(s, ..) = parse_macro_input!(input as ScoutData<NotExistMatcher>);
    quote! {
        #s
    }
    .into()
}

struct ValidFileMatcher;

impl Matcher for ValidFileMatcher {
    fn try_match(path: &Path, span: Span) -> syn::Result<()> {
        if let None = path.extension() {
            return Err(Error::new(span, "this path belongs to a directory"));
        }

        if path.is_absolute() {
            let root = Path::new("/");
            root.metadata().map_err(|_| {
                Error::new(
                    span,
                    format!(
                        "lacks permission to access parent at \"{}\"",
                        root.display()
                    ),
                )
            })?;
            Self::try_parent_exists(path, span)?;
            Self::try_absolute(path, span)
        } else {
            let abs_current_dir = std::env::current_dir().expect("current dir should be valid");
            abs_current_dir.metadata().map_err(|_| {
                Error::new(
                    span,
                    format!(
                        "lacks permission to access parent at \"{}\"",
                        abs_current_dir.display()
                    ),
                )
            })?;
            let abs_path = abs_current_dir.join(path);
            Self::try_parent_exists(&abs_path, span)?;
            Self::try_absolute(&abs_path, span)
        }
    }
}

#[proc_macro]
pub fn valid_file(input: TokenStream) -> TokenStream {
    let ScoutData(s, ..) = parse_macro_input!(input as ScoutData<ValidFileMatcher>);
    quote! {
        #s
    }
    .into()
}

struct ValidDirMatcher;

impl Matcher for ValidDirMatcher {
    fn try_match(path: &Path, span: Span) -> syn::Result<()> {
        if let Some(_) = path.extension() {
            return Err(Error::new(span, "this path belongs to a file"));
        }

        if path.is_absolute() {
            let root = Path::new("/");
            root.metadata().map_err(|_| {
                Error::new(
                    span,
                    format!(
                        "lacks permission to access parent at \"{}\"",
                        root.display()
                    ),
                )
            })?;
            Self::try_parent_exists(path, span)?;
            Self::try_absolute(path, span)
        } else {
            let abs_current_dir = std::env::current_dir().expect("current dir should be valid");
            abs_current_dir.metadata().map_err(|_| {
                Error::new(
                    span,
                    format!(
                        "lacks permission to access parent at \"{}\"",
                        abs_current_dir.display()
                    ),
                )
            })?;
            let abs_path = abs_current_dir.join(path);
            Self::try_parent_exists(&abs_path, span)?;
            Self::try_absolute(&abs_path, span)
        }
    }
}

#[proc_macro]
pub fn valid_dir(input: TokenStream) -> TokenStream {
    let ScoutData(s, ..) = parse_macro_input!(input as ScoutData<ValidDirMatcher>);
    quote! {
        #s
    }
    .into()
}

struct ValidDirAllMatcher;

impl Matcher for ValidDirAllMatcher {
    fn try_match(path: &Path, span: Span) -> syn::Result<()> {
        if let Some(_) = path.extension() {
            return Err(Error::new(span, "this path belongs to a file"));
        }

        if path.is_absolute() {
            let root = Path::new("/");
            root.metadata().map_err(|_| {
                Error::new(
                    span,
                    format!(
                        "lacks permission to access parent at \"{}\"",
                        root.display()
                    ),
                )
            })?;
            Self::try_absolute(path, span)
        } else {
            let abs_current_dir = std::env::current_dir().expect("current dir should be valid");
            abs_current_dir.metadata().map_err(|_| {
                Error::new(
                    span,
                    format!(
                        "lacks permission to access parent at \"{}\"",
                        abs_current_dir.display()
                    ),
                )
            })?;
            let abs_path = abs_current_dir.join(path);
            Self::try_absolute(&abs_path, span)
        }
    }
}

#[proc_macro]
pub fn valid_dir_all(input: TokenStream) -> TokenStream {
    let ScoutData(s, ..) = parse_macro_input!(input as ScoutData<ValidDirAllMatcher>);
    quote! {
        #s
    }
    .into()
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
