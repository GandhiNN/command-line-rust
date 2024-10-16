mod cli;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" }
    );
}
