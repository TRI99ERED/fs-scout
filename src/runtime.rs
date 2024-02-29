use std::path::{Component, Components, Path};

pub fn try_valid<P: AsRef<Path>>(path: P) -> Result<(), String> {
    let path = path.as_ref();

    if path.is_absolute() {
        try_absolute(path)
    } else {
        let abs_current_dir = std::env::current_dir().expect("current dir should be valid");
        let abs_path = abs_current_dir.join(path);
        try_absolute(&abs_path)
    }
}

fn try_absolute(path: &Path) -> Result<(), String> {
    let mut components = path.components();
    match components.next() {
        None => return Err("empty path".to_owned()),
        Some(comp) => match comp {
            Component::Prefix(_) => {
                if let Some(comp) = components.next() {
                    match comp {
                        Component::RootDir => {
                            return try_absolute_components(components, path);
                        }
                        _ => (),
                    }
                }
            }
            Component::RootDir => {
                return try_absolute_components(components, path);
            }
            _ => (),
        },
    }

    fn try_absolute_components(components: Components, path: &Path) -> Result<(), String> {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Move {
            Descend,
            Ascend,
        }

        let mut move_history = vec![];
        for component in components {
            match component {
                Component::Normal(name) => {
                    #[cfg(feature = "win")]
                    {
                        check_name(name.to_str().expect("path should be valid UTF-8"))?;
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
                return Err(format!(
                    "the path \"{}\" refers to an item outside the root",
                    path.display()
                ));
            }
        }
        Ok(())
    }

    Ok(())
}

#[cfg(feature = "win")]
fn check_name(name: &str) -> Result<(), String> {
    let invalid_trailing = regex::Regex::new(r".*[. ]$").unwrap();
    if invalid_trailing.is_match(name) {
        Err(format!(
            "invalid character in trailing position: '{}'",
            invalid_trailing
                .find(name)
                .unwrap()
                .as_str()
                .chars()
                .last()
                .expect("should be non-empty")
        ))
    } else {
        let invalid_sequences = regex::Regex::new(
            r#"[<>:"/\\|?*\x00-\x1F]|^(?i:CON|PRN|AUX|NUL|COM[0-9]|LPT[0-9])(?:\..+)?$"#,
        )
        .unwrap();
        if invalid_sequences.is_match(name) {
            Err(format!(
                "invalid character sequence: {}",
                invalid_sequences.find(name).unwrap().as_str()
            ))
        } else {
            Ok(())
        }
    }
}
