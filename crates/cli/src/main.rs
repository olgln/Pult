//! The `pult` command.
//!
//! Only `--help` and `--version` exist so far; the session commands come in later steps.

use std::io::{self, Write};
use std::process::ExitCode;

/// What the user asked for on the command line.
#[derive(Debug, PartialEq, Eq)]
enum Command {
    Help,
    Version,
}

/// A command line that could not be understood.
#[derive(Debug, PartialEq, Eq)]
enum ArgError {
    Unknown(String),
    TooMany,
}

impl std::fmt::Display for ArgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgError::Unknown(arg) => write!(f, "unknown argument '{arg}'"),
            ArgError::TooMany => write!(f, "expected at most one argument"),
        }
    }
}

/// Parses the arguments that follow the program name.
fn parse_args(args: &[String]) -> Result<Command, ArgError> {
    match args {
        [] => Ok(Command::Help),
        [arg] => match arg.as_str() {
            "-h" | "--help" | "help" => Ok(Command::Help),
            "-V" | "--version" | "version" => Ok(Command::Version),
            other => Err(ArgError::Unknown(other.to_string())),
        },
        _ => Err(ArgError::TooMany),
    }
}

fn help_text() -> String {
    format!(
        "pult {version}\n\
         Control panel for Claude Code sessions.\n\
         \n\
         Usage: pult [OPTION]\n\
         \n\
         Options:\n  \
         -h, --help     Print this help\n  \
         -V, --version  Print the version\n",
        version = pult_core::version()
    )
}

/// Runs the command line and returns the process exit code.
fn run(args: &[String], out: &mut dyn Write, err: &mut dyn Write) -> ExitCode {
    let written = match parse_args(args) {
        Ok(Command::Help) => out.write_all(help_text().as_bytes()),
        Ok(Command::Version) => writeln!(out, "pult {}", pult_core::version()),
        Err(e) => {
            // If stderr is gone there is nobody to tell.
            let _ = writeln!(err, "pult: {e}\nTry 'pult --help'.");
            return ExitCode::from(2);
        }
    };
    match written {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    run(&args, &mut io::stdout().lock(), &mut io::stderr().lock())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    fn run_captured(list: &[&str]) -> (ExitCode, String, String) {
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let code = run(&args(list), &mut out, &mut err);
        (
            code,
            String::from_utf8(out).unwrap(),
            String::from_utf8(err).unwrap(),
        )
    }

    /// A writer that always fails, like a closed pipe.
    struct Broken;

    impl Write for Broken {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::from(io::ErrorKind::BrokenPipe))
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn no_arguments_means_help() {
        assert_eq!(parse_args(&[]), Ok(Command::Help));
    }

    #[test]
    fn help_flags() {
        for flag in ["-h", "--help", "help"] {
            assert_eq!(parse_args(&args(&[flag])), Ok(Command::Help), "{flag}");
        }
    }

    #[test]
    fn version_flags() {
        for flag in ["-V", "--version", "version"] {
            assert_eq!(parse_args(&args(&[flag])), Ok(Command::Version), "{flag}");
        }
    }

    #[test]
    fn unknown_argument_is_an_error() {
        assert_eq!(
            parse_args(&args(&["--frobnicate"])),
            Err(ArgError::Unknown("--frobnicate".into()))
        );
    }

    #[test]
    fn more_than_one_argument_is_an_error() {
        assert_eq!(
            parse_args(&args(&["--help", "--version"])),
            Err(ArgError::TooMany)
        );
    }

    #[test]
    fn error_messages() {
        assert_eq!(
            ArgError::Unknown("x".into()).to_string(),
            "unknown argument 'x'"
        );
        assert_eq!(
            ArgError::TooMany.to_string(),
            "expected at most one argument"
        );
    }

    #[test]
    fn version_prints_name_and_version() {
        let (code, out, err) = run_captured(&["--version"]);
        assert_eq!(code, ExitCode::SUCCESS);
        assert_eq!(out, format!("pult {}\n", pult_core::version()));
        assert!(err.is_empty());
    }

    #[test]
    fn help_lists_the_options() {
        let (code, out, err) = run_captured(&["--help"]);
        assert_eq!(code, ExitCode::SUCCESS);
        assert!(out.starts_with(&format!("pult {}\n", pult_core::version())));
        assert!(out.contains("Usage: pult [OPTION]"));
        assert!(out.contains("-h, --help"));
        assert!(out.contains("-V, --version"));
        assert!(err.is_empty());
    }

    #[test]
    fn bad_arguments_exit_with_2_and_explain_on_stderr() {
        let (code, out, err) = run_captured(&["--nope"]);
        assert_eq!(code, ExitCode::from(2));
        assert!(out.is_empty());
        assert_eq!(err, "pult: unknown argument '--nope'\nTry 'pult --help'.\n");
    }

    #[test]
    fn write_failure_is_reported_in_the_exit_code() {
        let mut err = Vec::new();
        assert_eq!(
            run(&args(&["--version"]), &mut Broken, &mut err),
            ExitCode::FAILURE
        );
        assert_eq!(
            run(&args(&["--help"]), &mut Broken, &mut err),
            ExitCode::FAILURE
        );
        assert!(err.is_empty());
    }

    #[test]
    fn broken_stderr_still_exits_with_2() {
        let mut out = Vec::new();
        assert_eq!(
            run(&args(&["--nope"]), &mut out, &mut Broken),
            ExitCode::from(2)
        );
        let mut broken = Broken;
        assert!(broken.flush().is_ok());
    }
}
