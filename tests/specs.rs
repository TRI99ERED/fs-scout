#![allow(unused)]

use fs_scout::{exists, valid};

#[test]
fn example() {
    let file = exists!("Cargo.toml");
    let dir = exists!("fs-scout-macros/src\\");

    // let reserved_name = valid!("con");
    // let reserved_char = valid!("?");
    // let nul_char = valid!("\0");
    // let control_char = valid!("\n");
    // let trailing_dot = valid!("c.");
    // let trailing_space = valid!("c ");
    let leading_dot = valid!(".c");
    let disk_user_b = valid!("C:\\user\\docs\\Letter.txt");
    let disk_user_f = valid!("C:/user/docs/Letter.txt");
    let user_b = valid!("\\user\\docs\\Letter.txt");
    let user_f = valid!("/user/docs/Letter.txt");
    let disk_abs_b = valid!("C:\\Letter.txt");
    let disk_abs_f = valid!("C:/Letter.txt");
    let abs_b = valid!("\\Letter.txt");
    let abs_f = valid!("/Letter.txt");
    let unc_n = valid!("\\\\Server01\\user\\docs\\Letter.txt");
    let unc_d = valid!("\\\\?\\UNC\\Server01\\user\\docs\\Letter.txt");
    let disk_d = valid!("\\\\?\\C:\\user\\docs\\Letter.txt");
    let cur_b = valid!(".\\inthisdir");
    let cur_f = valid!("./inthisdir");
    let parent_b = valid!("..\\..\\greatgrandparent");
    let parent_f = valid!("../../greatgrandparent");
}
