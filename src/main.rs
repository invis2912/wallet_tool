// Include the lib.rs module
mod lib;

// Import necessary modules and functions from the lib
use crate::lib::cli::Args;
use crate::lib::network::{parse_network_and_key_type};
use crate::lib::client::{create_client};
use crate::lib::address::derive_addresses;

// Import additional libraries for CLI parsing and error handling
use figlet_rs::FIGfont;
use clap::Parser;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Print the ASCII art for Bitcoin Wallet Tool
    let standard_font = FIGfont::standard().unwrap();
    let wallet_tool = standard_font.convert("Bitcoin Wallet Tool");
    assert!(wallet_tool.is_some());
    println!("{}", wallet_tool.unwrap());

    // Parse command line arguments
    let args = Args::parse();

    // Parse network and key type from the provided extended public key
    let (network_type, key_type) = parse_network_and_key_type(&args.pubkey)?;

    // Create an HTTP client
    let client = create_client();

    // Determine the end index for address derivation, if provided
    let end_index = if args.end == 0 { None } else { Some(args.end) };

    // Derive addresses from the extended public key and get their balance and transaction count
    let address_infos = derive_addresses(&client, &args.pubkey, args.start, end_index, args.unused_threshold, &network_type, &key_type).await?;

    let mut total_balance = 0;
    let mut total_tx_count = 0;

    // Print the used addresses and their details
    for info in &address_infos {
        println!(
            "Address: {} - Balance: {} satoshis ({:.8} BTC) - Transactions: {}",
            info.address,
            info.balance,
            info.balance as f64 / 100_000_000.0,
            info.tx_count
        );
        total_balance += info.balance;
        total_tx_count += info.tx_count;
    }

    // Print the total
    println!("Total Used Addresses: {}", address_infos.len());
    println!(
        "Total Balance: {} satoshis ({:.8} BTC), Total Transactions: {}",
        total_balance,
        total_balance as f64 / 100_000_000.0,
        total_tx_count
    );

    println!("Thank you for using the Tool");

    Ok(())
}
