#![forbid(unsafe_code)]

use std::ffi::{OsStr, OsString};
use std::process::{Command, Output};

fn invoke(binary: &OsStr, args: &[&str]) -> Output {
    Command::new(binary)
        .args(args)
        .env("DATABASE_URL", "postgres://127.0.0.1:1/offline")
        .env("SHADOW_DATABASE_URL", "postgres://127.0.0.1:1/offline")
        .output()
        .unwrap_or_else(|error| panic!("failed to invoke {binary:?} with {args:?}: {error}"))
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("CLI stdout is UTF-8")
}

fn stderr(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).expect("CLI stderr is UTF-8")
}

fn assert_success(output: &Output, context: &str) {
    assert!(
        output.status.success(),
        "{context} failed with {:?}:\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        stdout(output),
        stderr(output)
    );
}

fn main() {
    let binary: OsString = std::env::args_os()
        .nth(1)
        .expect("pass the installed dpm binary as argument 1");

    let version = invoke(&binary, &["version"]);
    assert_success(&version, "dpm version");
    assert_eq!(stdout(&version).trim(), "dpm 0.3.2");

    let version_flag = invoke(&binary, &["--version"]);
    assert_success(&version_flag, "dpm --version");
    assert_eq!(stdout(&version_flag).trim(), "dpm 0.3.2");

    let help = invoke(&binary, &["help"]);
    assert_success(&help, "offline dpm help");
    let help_text = stdout(&help);
    assert!(help_text.contains("declarative postgres migrate"));
    assert!(help_text.contains("FLAGS (flags-2-env contract"));
    assert!(help_text.contains("DPM_ALLOW_DESTRUCTIVE"));

    let help_flag = invoke(&binary, &["--help"]);
    assert_success(&help_flag, "offline dpm --help");
    assert!(stdout(&help_flag).contains("DPM_ALLOW_DESTRUCTIVE"));

    let unknown = invoke(&binary, &["definitely-not-a-command"]);
    assert_eq!(unknown.status.code(), Some(1));
    assert!(stderr(&unknown).contains("unknown command"));

    let missing_source = invoke(&binary, &["diff"]);
    assert_eq!(missing_source.status.code(), Some(1));
    assert!(stderr(&missing_source).contains("no source"));
}
