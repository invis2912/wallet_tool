use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The extended public key (xpub/ypub/zpub/vpub)
    pub pubkey: String,

    /// The starting index for deriving addresses (optional)
    #[arg(default_value_t = 0)]
    pub start: u32,

    /// The ending index for deriving addresses (optional)
    #[arg(default_value_t = 0)]
    pub end: u32,

    /// The number of unused addresses in a row to stop derivation (optional)
    #[arg(short, long, default_value_t = 80)]
    pub unused_threshold: u32,
}
