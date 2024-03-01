//! This crate is an experiment! It can't guarantee correctness in all usecases.

extern crate fs_scout_macros;

pub use fs_scout_macros::*;

#[macro_export]
macro_rules! create_file {
    ($path:literal!) => {{
        std::fs::File::create(fs_scout::valid_file!($path))
            .expect("should be able to create a file")
    }};
    ($path:literal) => {{
        std::fs::File::create(fs_scout::valid_file!($path))
    }};
}

#[macro_export]
macro_rules! create_new_file {
    ($path:literal!) => {{
        fs_scout::not_exists!($path);
        std::fs::File::options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(fs_scout::valid_file!($path))
            .expect("should be able to create a new file")
    }};
    ($path:literal) => {{
        fs_scout::not_exists!($path);
        std::fs::File::options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(fs_scout::valid_file!($path))
    }};
}

#[macro_export]
macro_rules! open_file {
    ($path:literal!) => {{
        fs_scout::exists!($path);
        std::fs::File::open(fs_scout::valid_file!($path)).expect("should be able to open a file")
    }};
    ($path:literal) => {{
        fs_scout::exists!($path);
        std::fs::File::open(fs_scout::valid_file!($path))
    }};
}

#[macro_export]
macro_rules! read_file {
    ($path:literal!) => {{
        fs_scout::exists!($path);
        &std::fs::read(fs_scout::valid_file!($path)).expect("should be able to read a file")
    }};
    ($path:literal) => {{
        fs_scout::exists!($path);
        &std::fs::read(fs_scout::valid_file!($path))
    }};
}

#[macro_export]
macro_rules! write_file {
    ($path:literal!, $contents:expr) => {{
        std::fs::write(fs_scout::valid_file!($path), $contents)
            .expect("should be a ble to write to a file")
    }};
    ($path:literal, $contents:expr) => {{
        std::fs::write(fs_scout::valid_file!($path), $contents)
    }};
}

#[macro_export]
macro_rules! create_dir {
    ($path:literal!) => {{
        fs_scout::not_exists!($path);
        std::fs::create_dir(fs_scout::valid_dir!($path))
            .expect("should be able to create a directory")
    }};
    ($path:literal) => {{
        fs_scout::not_exists!($path);
        std::fs::create_dir(fs_scout::valid_dir!($path))
    }};
}

#[macro_export]
macro_rules! create_dir_all {
    ($path:literal!) => {{
        std::fs::create_dir_all(fs_scout::valid_dir_all!($path))
            .expect("should be able to create all directories")
    }};
    ($path:literal) => {{
        std::fs::create_dir_all(fs_scout::valid_dir_all!($path))
    }};
}
