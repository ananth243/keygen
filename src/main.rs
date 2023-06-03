mod password;
use clap::Parser;
use password::generate_password;
use copypasta::{ClipboardContext, ClipboardProvider};

#[derive(Parser, Debug)]
#[command(author="Ananth Raghav", version, about="A command line password generator", long_about = None)]

struct Args {
    /// Include uppercase alphabets. Default false
    #[arg(short, long)]
    alpha: Option<bool>,

    /// Length of password
    #[arg(short, long)]
    length: Option<u8>,
}

fn main() {
    let args = Args::parse();
    let l = match args.length{
        None=> 8,
        Some(t)=> t
    };
    let alpha = match args.alpha{
        None=>false,
        Some(t)=>t
    };
    let password = generate_password(l,alpha);
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(password.as_str().to_owned()).unwrap();
    ctx.get_contents().unwrap();
    println!("Password copied to clipboard!");
}