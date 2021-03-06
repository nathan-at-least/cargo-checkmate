use crate::executable::Executable;
use crate::IOResult;
use enum_iterator::IntoEnumIterator;
use std::fmt;
use structopt::StructOpt;

#[derive(Debug, IntoEnumIterator, StructOpt)]
pub enum Check {
    /// Run all checks.
    Everything,

    /// check: `cargo check` syntax + type checking.
    Check,

    /// check: Use `cargo fmt` to check if code is correctly formatted.
    Format,

    /// check: `cargo build` the default target.
    Build,

    /// check: `cargo test` automated unit tests.
    Test,

    /// check: `cargo doc` generation.
    Doc,

    /// check: `cargo audit` security advisories across all dependencies.
    Audit,
}

impl Check {
    fn execute_everything(&self) -> IOResult<()> {
        use crate::runner::Runner;

        let mut runner = Runner::new()?;

        println!(
            "\nrunning {} {} phases",
            Check::VARIANT_COUNT - 1,
            crate::CMDNAME
        );

        for check in Check::into_enum_iter() {
            match check {
                Check::Everything => {}
                _ => runner.run_check(&format!("{}", check))?,
            }
        }

        runner.exit()
    }
}

impl Executable for Check {
    fn execute(&self) -> IOResult<()> {
        use crate::subcommands::{audit, cargo_builtin};

        match self {
            Check::Everything => self.execute_everything(),
            Check::Audit => audit(),
            Check::Build => cargo_builtin(&["build"]),
            Check::Check => cargo_builtin(&["check"]),
            Check::Doc => cargo_builtin(&["doc"]),
            Check::Format => cargo_builtin(&["fmt", "--", "--check"]),
            Check::Test => cargo_builtin(&["test"]),
        }
    }
}

impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dbg = format!("{:?}", self);
        write!(f, "{}", dbg.to_lowercase())
    }
}
