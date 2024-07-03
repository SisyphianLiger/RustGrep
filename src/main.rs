use minigrep::run;
use minigrep::Config;
use std::env;
use std::ffi::OsString;
use std::process;

// Let's process some Args
fn main() {
    // Using the env lib we are taking in args from the command line
    // env::arg panics if there is invalid unicode
    let args: Vec<OsString> = env::args_os().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Prints out the following text: poem.txt
    if let Err(e) = run(&config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    // And using the debug trait printing them out
    // dbg!(args);
}
