use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os.as_str() == "macos" {
        env::set_var("PIXELS_HIGH_PERF", "1");
    }
}
