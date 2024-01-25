use cfg_if::cfg_if;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

cfg_if! {
    if #[cfg(unix)] {
        const EXE_EXT: &str = "";
        const LIB_EXT: &str = ".so";
    } else if #[cfg(windows)] {
        const EXE_EXT: &str = ".exe";
        const LIB_EXT: &str = ".dll";
    } else {
        unimplemented!("building for current target is not supported yet");
    }
}

#[derive(Debug)]
pub(crate) struct BuildOpt {
    name: String,
    pub release: bool,
    pub lib: bool,
    pub target: String,
    pub target_dir: Option<PathBuf>,
    dest_dir: PathBuf,
    cargo_pkg_name: String,
}

impl BuildOpt {
    pub fn new(name: String) -> Self {
        let (cargo_pkg_name, dest_dir) = update_name_(&name);
        Self {
            name,
            release: false,
            lib: true,
            target: env!("TARGET").into(),
            target_dir: None,
            dest_dir,
            cargo_pkg_name,
        }
    }

    pub fn set_name(&mut self, name: String) {
        let (cargo_pkg_name, dest_dir) = update_name_(&name);
        self.name = name;
        self.cargo_pkg_name = cargo_pkg_name;
        self.dest_dir = dest_dir;
    }

    pub fn is_dummy(&self) -> bool {
        self.name.is_empty()
    }

    /// Run `cargo build` command with provided options,
    /// then copy the artifacts into destinate directory.
    ///
    /// # Panic
    /// Will panic if:
    /// 1. The build command fail.
    /// 2. The built artifacts cannot be found.
    /// 3. The destinate directory cannot be created.
    /// 4. Lack of permission to copy files into destinate directory.
    pub fn cargo_build(&self) {
        let mut cargo = Command::new("cargo");
        let mut cmd = cargo.args(["build", "-p", &self.cargo_pkg_name, "--target", &self.target]);
        let mut artifact_dir_name = "debug";

        if self.release {
            cmd = cmd.arg("--release");
            artifact_dir_name = "release"
        }
        if let Some(tg_dir) = &self.target_dir {
            cmd = cmd.arg("--target-dir").arg(tg_dir.as_os_str());
        }

        if !cmd
            .status()
            .unwrap_or_else(|err| {
                panic!(
                    "{}",
                    format!(
                        "unable to execute `cargo build` for package '{}': {err}",
                        &self.name
                    )
                )
            })
            .success()
        {
            panic!("failed to build package '{}'", &self.name);
        }

        // Move the artifacts into dest dir
        let src_file_name = if self.lib {
            format!("lib{}{LIB_EXT}", self.cargo_pkg_name)
        } else {
            format!("{}{EXE_EXT}", self.cargo_pkg_name)
        };
        let src_path = self
            .target_dir
            .clone()
            .unwrap_or_else(|| PathBuf::from("target"))
            .join(&self.target)
            .join(artifact_dir_name)
            .join(src_file_name);
        fs::create_dir_all(&self.dest_dir)
            .unwrap_or_else(|_| panic!("unable to create directory: {}", self.dest_dir.display()));
        copy_file_under(&src_path, &self.dest_dir)
            .expect("unable to copy artifacts to destinate folder");
    }
}

fn update_name_(name: &str) -> (String, PathBuf) {
    // Differentiate godot project dir and rust source code dir by cases.
    // Godot project dir uses kabab-case as name, while cargo pkgs uses snake_case as name.
    // Doint these transformations to make sure both way works.
    let gd_proj_name = name.replace('_', "-");
    let cargo_pkg_name = name.replace('-', "_");

    let target = env!("TARGET").to_string();
    let dest_dir = PathBuf::from(env!("CARGO_WORKSPACE_DIR"))
        .join(gd_proj_name)
        .join("Lib")
        .join(&target);
    (cargo_pkg_name, dest_dir)
}

fn copy_file_under(src: &Path, dir: &Path) -> Result<(), std::io::Error> {
    let src_filename = src
        .file_name()
        .ok_or_else(|| std::io::Error::other("missing src file"))?;
    fs::copy(src, dir.join(src_filename))?;
    Ok(())
}
