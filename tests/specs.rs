#![allow(unused)]

use std::fs::{create_dir, read};

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
    // let trailing_dot = valid_file!("any.");
    // let trailing_space = valid_dir!("any ");

    let abs_file_b = valid_file!("\\Letter.txt");
    let abs_file_f = valid_file!("/Letter.txt");
    let abs_dir_b = valid_dir!("\\user");
    let abs_dir_f = valid_dir!("/user");

    let disk_abs_file_b = valid_file!("C:\\Letter.txt");
    let disk_abs_file_f = valid_file!("C:/Letter.txt");
    let disk_abs_dir_b = valid_dir!("C:\\user");
    let disk_abs_dir_f = valid_dir!("C:/user");

    let unc_disk_file = valid_file!("\\\\?\\C:\\Letter.txt\\");
    let unc_disk_dir = valid_dir!("\\\\?\\C:\\user\\");

    let cur_file_b = valid_file!(".\\cur.bin");
    let cur_file_f = valid_file!("./cur.bin");
    let cur_dir_b = valid_dir!(".\\cur");
    let cur_dir_f = valid_dir!("./cur");

    let parent_file_b = valid_file!("..\\parent.bin");
    let parent_file_f = valid_file!("../parent.bin");
    let parent_dir_b = valid_dir!("..\\parent");
    let parent_dir_f = valid_dir!("../parent");

    let hidden_dir = valid_dir!(".hidden");
    let hidden_file = valid_file!(".hidden.bin");

    let here_dir = valid_dir!("dir");
    let here_file = valid_file!("file.bin");

    let created_file = create_file!("file.bin"!);
    let opened_file = open_file!("Cargo.toml"!);
    // let invalid_opened_file = open!("file.bin"!);
    let read_file = read_file!("Cargo.toml"!);
    // let invalid_read_file = read!("file.bin"!);
    let writen_file = write_file!("written_file.bin"!, "some text to write");
    let created_dir = create_dir!("dir"!);
    // let invalid_created_dir = create_dir!("src"!);
    let created_dir_all = create_dir_all!("some/dir/tree"!);

    println!("{}", String::from_utf8_lossy(read_file));
}
