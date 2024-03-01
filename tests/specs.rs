#![allow(unused)]

use std::fs::create_dir;

use fs_scout::*;

#[test]
fn example() {
    let file = exists!("Cargo.toml");
    let dir = exists!("fs-scout-macros/src\\");
    // let non_existant = exists!("a");

    // let reserved_name = valid_dir!("com1");
    // let reserved_char = valid_dir!("?");
    // let nul_char = valid_dir!("\0");
    // let control_char = valid_dir!("\n");
    // let trailing_dot = valid_dir!("c.");
    // let trailing_space = valid_dir!("c ");
    let hidden = valid_dir!(".c");
    let disk_abs_b = valid_file!("C:\\Letter.txt");
    let disk_abs_f = valid_file!("C:/Letter.txt");
    let abs_b = valid_file!("\\Letter.txt");
    let abs_f = valid_file!("/Letter.txt");
    let unc_d_disk = valid_dir!("\\\\?\\C:\\user\\");
    let cur_b = valid_dir!(".\\inthisdir");
    let cur_f = valid_dir!("./inthisdir");
    let parent_b = valid_dir!("..\\..\\greatgrandparent");
    let parent_f = valid_dir!("../../greatgrandparent");
    let here_dir = valid_dir!("dir");
    let here_file = valid_file!("file.bin");

    let created_file = create_file!("file.bin"!);
    let created_new_file = create_new_file!("file.bin"!);
    // let invalid_new_file = create_new!("Cargo.toml"!);
    let opened_file = open_file!("Cargo.toml"!);
    // let invalid_opened_file = open!("file.bin"!);
    let read_file = read_file!("Cargo.toml"!);
    // let invalid_read_file = read!("file.bin"!);
    let writen_file = write_file!("file.bin"!, "some text to write");
    let created_dir = create_dir!("dir"!);
    // let invalid_created_dir = create_dir!("src"!);
    let created_dir_all = create_dir_all!("/some/dir/tree"!);
}
