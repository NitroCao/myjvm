use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// classpath of bootstrap class
    #[arg(short = 'j', long)]
    jrepath: Option<String>,
    /// classpath of Java command
    #[arg(short = 'c', long)]
    classpath: Option<String>,
    /// main class to run
    class: String,
    /// optional arguments
    args: Vec<String>,
}

impl Args {
    pub fn classpath(&self) -> &Option<String> {
        &self.classpath
    }

    pub fn class(&self) -> &String {
        &self.class
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }
}
