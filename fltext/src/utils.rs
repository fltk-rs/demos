use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

pub fn strip_unc_path(p: &Path) -> String {
    let p = p.to_str().unwrap();
    if let Some(end) = p.strip_prefix("\\\\?\\") {
        end.to_string()
    } else {
        p.to_string()
    }
}

#[allow(dead_code)]
pub fn has_program(prog: &str) -> bool {
    // hacky
    match Command::new(prog).arg("--version").output() {
        Ok(out) => !out.stdout.is_empty(),
        _ => false,
    }
}

pub fn init_args(args: env::Args) -> (Option<PathBuf>, PathBuf) {
    let args: Vec<_> = args.collect();
    let mut current_file: Option<PathBuf> = None;
    // fix our working dir
    if args.len() > 1 {
        let path = PathBuf::from(args[1].clone());
        if path.exists() {
            if path.is_dir() {
                env::set_current_dir(path.clone()).unwrap();
            } else {
                current_file = Some(PathBuf::from(path.file_name().unwrap()));
                if let Some(parent) = path.parent() {
                    if parent.exists() {
                        env::set_current_dir(parent).unwrap();
                    }
                }
            }
        }
        path
    } else {
        env::current_dir().unwrap()
    };

    let current_path = env::current_dir().unwrap().canonicalize().unwrap();
    (current_file, current_path)
}

#[allow(dead_code)]
pub fn can_use_xterm() -> bool {
    if cfg!(not(any(target_os = "macos", target_os = "windows"))) {
        if let Ok(var) = env::var("XDG_SESSION_TYPE") {
            var == "x11" && has_program("xterm")
        } else {
            env::var("RED_XTERM").is_ok()
        }
    } else {
        false
    }
}
