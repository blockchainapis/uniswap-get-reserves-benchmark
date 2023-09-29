use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    version = "0.1",
    author = "Blockchain APIs",
    about = "This program compare the speed of download of blocks of my personal node vs Cloudflare."
)]
pub struct Args {
    /// The amount of pairs that you are willing to use
    #[arg(short = 'a', long)]
    pub pair_amount: u64,

    /// The amount of threads to use
    #[arg(short = 't', long)]
    pub threads: usize
}
