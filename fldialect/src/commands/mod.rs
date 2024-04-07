use std::{env, process::Command};

pub fn run(tts: bool, from: String, to: String, word: String) -> String {
    if cfg!(target_family = "unix") {
        let mode = match word.split_whitespace().count() {
            1 => "",
            _ => "-brief",
        };
        let speak = match tts {
            true => "-speak",
            false => "",
        };
        let run = Command::new("bash")
            .arg("-xc")
            .arg(format!("trans {mode} -join-sentence -no-ansi {speak} -show-languages n -show-original n -show-original-dictionary n -show-original-dictionary n -show-prompt-message n -show-alternatives n -show-translation-phonetics n -indent 2 -from {from} -to {to} << 'EOF'\n{}\nEOF", word.trim().replace("\n\n", "\n")))
            .output()
            .expect("failed to execute bash");
        match run.status.success() {
            true => String::from_utf8_lossy(&run.stdout).to_string(),
            false => panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr)),
        }
    } else {
        "no way".to_string()
    }
}

pub fn list() -> String {
    if cfg!(target_family = "unix") {
        let run = Command::new("trans")
            .arg("-list-languages-english")
            .output()
            .expect("failed to execute bash");
        match run.status.success() {
            true => String::from_utf8_lossy(&run.stdout)
                .split_whitespace()
                .map(str::to_string)
                .collect::<Vec<String>>()
                .join("|"),
            false => panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr)),
        }
    } else {
        "no way".to_string()
    }
}

pub fn once() -> bool {
    if cfg!(target_os = "linux") {
        let run = Command::new("bash")
            .arg("-xc")
            .arg(format!("lsof -t {}", env::current_exe().unwrap().display()))
            .output()
            .expect("failed to execute bash");
        match run.status.success() {
            true => {
                String::from_utf8_lossy(&run.stdout)
                    .split_whitespace()
                    .count()
                    == 1
            }
            false => panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr)),
        }
    } else {
        true
    }
}
