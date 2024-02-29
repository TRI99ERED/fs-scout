pub use fs_scout_macros::*;

#[macro_export]
macro_rules! create {
    ($path:literal) => {{
        std::fs::File::create(fs_scout::compiletime::valid_file!($path)).unwrap()
    }};
}

// TODO not_exist!()
#[macro_export]
macro_rules! create_new {
    ($path:literal) => {};
}

#[macro_export]
macro_rules! open {
    ($path:literal) => {
        std::fs::File::open(fs_scout::compiletime::exists!(valid_file!($path))).unwrap()
    };
}

#[macro_export]
macro_rules! read {
    ($path:literal) => {
        String::from_utf8_lossy(
            &std::fs::read(fs_scout::compiletime::exists!(valid_file!($path))).unwrap(),
        )
    };
}

#[macro_export]
macro_rules! write {
    ($path:literal, $contents:expr) => {{
        std::fs::write(fs_scout::compiletime::valid_file!($path), $contents).unwrap()
    }};
}

#[macro_export]
macro_rules! create_dir {
    ($path:literal) => {
        std::fs::create_dir(fs_scout::compiletime::valid_dir!($path)).unwrap()
    };
}

#[macro_export]
macro_rules! create_dir_all {
    ($path:literal) => {{
        std::fs::create_dir_all(fs_scout::compiletime::valid_dir_all!($path)).unwrap()
    }};
}
