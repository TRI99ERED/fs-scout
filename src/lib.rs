//! A simple crate exporting macros meant to help check the inputs to some [`std::fs`] functionality at compiletime.
//!
//! This crate is an `experiment`! It can't guarantee correctness in all usecases.
//!
//! # Features
//! The feature flag `"win"` enables checks for allowed names in Windows. The reason, this is not tied to
//! your OS is to support crossplatform development (e.g., knowing, that your program will not create any
//! files or directories forbidden by Windows). It is included in default feature for that very reason.
//! If you don't plan to deploy to Windows and need to bypass this restriction, you can use flag
//! `default-features = false` to disable this behaviour.

extern crate fs_scout_macros;

pub use fs_scout_macros::*;

/// Creates a file using [`std::fs::File::create`]. During compiletime validates the input path.
///
/// # Examples
/// ```rust, no_run
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use fs_scout::create_file;
///
/// let result: std::io::Result<File> = create_file!("file.txt");
///
/// // (!) This is practically equivalent to calling unwrap() on result above. Use at your own discretion.
/// let file: File = create_file!("file.txt"!);
/// #   Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! create_file {
    ($path:literal!) => {{
        std::fs::File::create($crate::valid_file!($path))
            .expect(&format!("should be able to create a file at {}", $path))
    }};
    ($path:literal) => {{
        std::fs::File::create($crate::valid_file!($path))
    }};
}

/// Opens a file using [`std::fs::File::open`]. During compiletime validates the input path.
///
/// # Examples
/// ```rust, ignore
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use fs_scout::open_file;
///
/// let result: std::io::Result<File> = open_file!("file.txt");
///
/// // (!) This is practically equivalent to calling unwrap() on result above. Use at your own discretion.
/// let file: File = open_file!("file.txt"!);
/// #   Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! open_file {
    ($path:literal!) => {{
        $crate::exists!($path);
        std::fs::File::open($crate::valid_file!($path))
            .expect(&format!("should be able to open a file at {}", $path))
    }};
    ($path:literal) => {{
        $crate::exists!($path);
        std::fs::File::open($crate::valid_file!($path))
    }};
}

/// Reads a file using [`std::fs::File::open`]. During compiletime validates the input path.
///
/// # Examples
/// ```rust, ignore
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use fs_scout::read_file;
///
/// let result: std::io::Result<Vec<u8>> = read_file!("file.txt");
/// let s = String::from_utf8_lossy(&result?);
///
/// // (!) This is practically equivalent to calling unwrap() on result above. Use at your own discretion.
/// let vector: Vec<u8> = read_file!("file.txt"!);
/// let s = String::from_utf8_lossy(&vector);
/// #   Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! read_file {
    ($path:literal!) => {{
        $crate::exists!($path);
        &std::fs::read($crate::valid_file!($path))
            .expect(&format!("should be able to read a file at {}", $path))
    }};
    ($path:literal) => {{
        $crate::exists!($path);
        &std::fs::read($crate::valid_file!($path))
    }};
}

/// Writes to a file using [`std::fs::File::create`]. During compiletime validates the input path.
///
/// # Examples
/// ```rust, no_run
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use fs_scout::write_file;
///
/// let result: std::io::Result<()> = write_file!("file.txt", "some text to write");
///
/// // (!) This is practically equivalent to calling unwrap() on result above. Use at your own discretion.
/// write_file!("file.txt"!, "some text to write");
/// #   Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! write_file {
    ($path:literal!, $contents:expr) => {{
        std::fs::write($crate::valid_file!($path), $contents)
            .expect(&format!("should be a ble to write to a file at {}", $path))
    }};
    ($path:literal, $contents:expr) => {{
        std::fs::write($crate::valid_file!($path), $contents)
    }};
}

/// Creates a directory using [`std::fs::create_dir_all`]. During compiletime validates the input path.
/// Checks, if the directory's ancestors exist.
///
/// [`std::fs::create_dir`] is not used, because it errors, if the directory already exists.
/// If you need to use it, call `std::fs::create_dir(fs_scout::valid!("..."))` instead and handle the error.
///
/// # Examples
/// ```rust, no_run
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use fs_scout::create_dir;
///
/// let result: std::io::Result<()> = create_dir!("some_dir");
///
/// // (!) This is practically equivalent to calling unwrap() on result above. Use at your own discretion.
/// create_dir!("some_dir"!);
/// #   Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! create_dir {
    ($path:literal!) => {{
        std::fs::create_dir_all($crate::valid_dir!($path))
            .expect(&format!("should be able to create directory at {}", $path))
    }};
    ($path:literal) => {{
        std::fs::create_dir_all($crate::valid_dir!($path))
    }};
}

/// Creates all missing directory in a path using [`std::fs::create_dir_all`]. During compiletime validates the input path.
///
/// # Examples
/// ```rust, no_run
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use fs_scout::create_dir_all;
///
/// let result: std::io::Result<()> = create_dir_all!("some/dir/tree");
///
/// // (!) This is practically equivalent to calling unwrap() on result above. Use at your own discretion.
/// create_dir_all!("some/dir/tree"!);
/// #   Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! create_dir_all {
    ($path:literal!) => {{
        std::fs::create_dir_all($crate::valid_dir_all!($path)).expect(&format!(
            "should be able to create all directories at {}",
            $path
        ))
    }};
    ($path:literal) => {{
        std::fs::create_dir_all($crate::valid_dir_all!($path))
    }};
}
