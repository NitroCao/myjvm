use clap::Parser;
mod cmd;

fn main() {
    let args = cmd::Args::parse();

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
