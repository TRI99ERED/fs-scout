#![allow(unused)]

use fs_scout::{scout, validate};

#[test]
fn example() {
    let valid = scout!("Cargo.toml");

    let p = scout!("fs-scout-macros/src\\");

    // let p = validate!("con");
}
