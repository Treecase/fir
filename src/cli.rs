//! Command-line argument parsing and responses.

/// Functions for printing help/usage/version/etc. information.
pub mod print {
    use crate::meta::{NAME, VERSION};

    /// Print help information.
    pub fn help() {
        println!(
            "\
    Usage: {NAME} [OPTION]... IMAGE...
    Display an image.
    Example: {NAME} some-image.png

    Options:
      -h, --help        display this help text and exit
      -v, --version     display version information and exit"
        );
    }

    /// Print usage information.
    pub fn usage() {
        println!(
            "\
    Usage: {NAME} [OPTION]... IMAGE...
    Try '{NAME} --help' for more information."
        )
    }

    /// Print version information.
    pub fn version() {
        println!("{NAME} {VERSION}");
    }
}

/// Argument parsing.
pub mod args {

    // --- Error --- ///////////////////////////////////////

    /// Argument parsing error conditions.
    #[derive(Clone, Copy, Debug)]
    pub enum Error {
        /// User did not supply a required argument.
        MissingArgument,
        /// User passed an unrecognized option.
        UnrecognizedOption,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::MissingArgument => write!(f, "missing argument"),
                Self::UnrecognizedOption => write!(f, "invalid option"),
            }
        }
    }

    impl std::error::Error for Error {}

    // --- Request --- /////////////////////////////////////

    /// Results of argument parsing.
    #[derive(Clone, Debug)]
    pub enum Request {
        /// User passed the "help" option.
        Help,
        /// User passed the "version" option.
        Version,
        /// Everything is good, the user passed some paths.
        View { files: Vec<std::path::PathBuf> },
    }

    // --- Functions --- ///////////////////////////////////

    /// Parse command-line arguments and return a result.
    ///
    /// This function expects that the iterator contains only the actual arguments; ie. the first
    /// element is not the executable name.
    pub fn parse(args: impl Iterator<Item = String>) -> Result<Request, Error> {
        let mut files = vec![];

        for arg in args {
            match arg.as_str() {
                "-h" | "--help" => {
                    return Ok(Request::Help);
                }
                "-v" | "--version" => {
                    return Ok(Request::Version);
                }
                a if a.starts_with("-") => {
                    return Err(Error::UnrecognizedOption);
                }
                _ => files.push(arg.into()),
            }
        }
        if files.is_empty() {
            Err(Error::MissingArgument)
        } else {
            Ok(Request::View { files })
        }
    }
}
