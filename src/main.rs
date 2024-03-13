use clap::Parser;

use crate::classpath::{direntry::new_direntry, classpath::ClasspathEntry};
mod cmd;
pub mod classpath;

fn main() {
    let args = cmd::Args::parse();

    let entry = new_direntry("/foo");
    println!("{}", entry.string());

    println!("main class: {}", args.class());
    match args.classpath() {
        Some(classpath) => {
            println!("classpath: {}", classpath);
        }
        None => {}
    };
    println!("optional args:");
    for arg in args.args().iter() {
        println!("\t{}", arg);
    }
}
