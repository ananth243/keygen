mod password;
use clap::Parser;
use password::generate_password;
use copypasta::{ClipboardContext, ClipboardProvider};

#[derive(Parser, Debug)]
#[command(author=env!("CARGO_PKG_AUTHORS"), version=env!("CARGO_PKG_VERSION"), about=env!("CARGO_PKG_DESCRIPTION"), long_about = None)]

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
    let l = args.length.unwrap_or(8);
    let alpha = args.alpha.unwrap_or(false);
    let password = generate_password(l,alpha);
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(password.as_str().to_owned()).unwrap();
    ctx.get_contents().unwrap();
    println!("Password copied to clipboard!");
}