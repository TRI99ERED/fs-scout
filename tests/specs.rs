#![allow(unused)]

use fs_scout::compiletime::{exists, valid_file, valid_dir, valid_dir_all};

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
    let leading_dot = valid_dir!(".c");
    let disk_abs_b = valid_file!("C:\\Letter.txt");
    let disk_abs_f = valid_file!("C:/Letter.txt");
    let abs_b = valid_file!("\\Letter.txt");
    let abs_f = valid_file!("/Letter.txt");
    let unc_d_disk = valid_dir!("\\\\?\\C:\\user\\");
    let cur_b = valid_dir!(".\\inthisdir");
    let cur_f = valid_dir!("./inthisdir");
    let parent_b = valid_dir!("..\\..\\greatgrandparent");
    let parent_f = valid_dir!("../../greatgrandparent");
}
