use std::{env, process::Command};

pub fn run(voice: bool, from: String, to: String, word: String) -> String {
    let run = Command::new("trans")
        .args([
            "-join-sentence",
            "-no-ansi",
            "-show-languages",
            "n",
            "-show-original",
            "n",
            "-show-original-dictionary",
            "n",
            "-show-original-dictionary",
            "n",
            "-show-prompt-message",
            "n",
            "-show-alternatives",
            "n",
            "-show-translation-phonetics",
            "n",
            "-indent",
            "2",
            "-from",
            &from,
            "-to",
            &to,
            match word.split_whitespace().count() {
                1 => "",
                _ => "-brief",
            },
            if voice { "-speak" } else { "" },
            &word.trim().replace("\n\n", "\n"),
        ])
        .output()
        .expect("failed to execute bash");
    String::from_utf8_lossy(match run.status.success() {
        true => &run.stdout,
        false => &run.stderr,
    })
    .to_string()
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
        let run = Command::new("lsof")
            .args(["-t", env::current_exe().unwrap().to_str().unwrap()])
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
