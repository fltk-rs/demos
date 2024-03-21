#![forbid(unsafe_code)]

mod commands;
mod components;
mod constants;
mod elements;
mod pages;

fn main() {
    if commands::once() {
        pages::main()
    }
}
