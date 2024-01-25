mod build;

use std::env;
use std::path::PathBuf;

use build::BuildOpt;

static USAGE: &str = "
Usage:
    cargo dev [COMMAND] <OPTION> [PROJECT]
    
Commands:
    build   - Execute debuging build for the provided project.
    release - Execute releaseing build for the provided project.

Options:
    -h, --help
            - Show this help message.
    --target-dir
            - Specify another directory to store built artifacts.
";

fn main() {
    let mut args = env::args().skip(1);
    // build options with dummy package name.
    let mut build_opt = BuildOpt::new(String::new());

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "build" => build_opt.release = false,
            "release" => build_opt.release = true,
            "--help" => {
                println!("{USAGE}");
                return;
            }
            "--target-dir" => build_opt.target_dir = args.next().map(PathBuf::from),
            _ => build_opt.set_name(arg)
        }
    }

    if build_opt.is_dummy() {
        println!("Missing project's name.\n{USAGE}");
        return;
    }

    build_opt.cargo_build();
}
