// Import necessary Bitcoin utilities for BIP32 child number and derivation paths
use bitcoin::util::bip32::{ChildNumber, DerivationPath};
use bitcoin::network::constants::Network;
use bitcoin::Address;
use bitcoin::secp256k1::Secp256k1;
use reqwest::Client;
use std::error::Error;
use std::sync::atomic::Ordering;

// Import custom modules and constants
use crate::lib::network::{NetworkType, ExtendedKeyType, ExtendedKeyConverter};
use crate::lib::constants::{API_CALL_COUNT, MAX_API_CALLS};
use crate::lib::error::CustomError;

// Struct to hold information about a derived address
pub struct AddressInfo {
    pub address: String,
    pub balance: u64,
    pub tx_count: u64,
}

// Function to fetch address information (balance and transaction count) from the blockchain
async fn get_address_info(client: &Client, address: &str, network_type: &NetworkType) -> Result<(u64, u64), CustomError> {
    // Determine the URL to fetch address information based on the network type
    let url = match network_type {
        NetworkType::Testnet => format!("https://blockstream.info/testnet/api/address/{}", address),
        NetworkType::Mainnet => format!("https://blockstream.info/api/address/{}", address),
    };

    // Variables for retry logic
    let mut attempts = 0;
    let max_attempts = 5;
    let mut backoff = 1;

    // Retry logic for fetching address information
    while attempts < max_attempts {
        match client.get(&url).send().await {
            // If the request is successful, parse the JSON response
            Ok(response) => match response.json::<serde_json::Value>().await {
                Ok(info) => {
                    let balance = info["chain_stats"]["funded_txo_sum"].as_u64().unwrap_or(0)
                        - info["chain_stats"]["spent_txo_sum"].as_u64().unwrap_or(0);
                    let tx_count = info["chain_stats"]["tx_count"].as_u64().unwrap_or(0);
                    return Ok((balance, tx_count));
                },
                // Handle JSON parsing errors
                Err(e) => {
                    eprintln!("Error parsing response JSON for address {}: {}", address, e);
                    return Err(CustomError::ReqwestError(e));
                },
            },
            // Handle request errors and implement exponential backoff
            Err(e) => {
                attempts += 1;
                eprintln!("Error fetching address info for {}: {}. Attempt {}/{}", address, e, attempts, max_attempts);
                if attempts < max_attempts {
                    tokio::time::sleep(tokio::time::Duration::from_secs(backoff)).await;
                    backoff *= 2;
                } else {
                    return Err(CustomError::ReqwestError(e));
                }
            },
        }
    }

    // Return an error if the maximum number of retries is reached
    Err(CustomError::MaxRetriesReached)
}

// Function to derive addresses from an extended public key and fetch their information
pub async fn derive_addresses(
    client: &Client,
    pubkey: &str, 
    start: u32, 
    end: Option<u32>, 
    unused_threshold: u32, 
    network_type: &NetworkType, 
    key_type: &ExtendedKeyType,
) -> Result<Vec<AddressInfo>, Box<dyn Error>> {
    // Determine the Bitcoin network based on the network type
    let network = match network_type {
        NetworkType::Mainnet => Network::Bitcoin,
        NetworkType::Testnet => Network::Testnet,
    };

    // Convert the extended public key to an xpub format
    let extended_pubkey = key_type.convert_to_xpub(pubkey)?;

    // Create a new Secp256k1 context
    let secp = Secp256k1::new();
    // Vector to hold information about derived addresses
    let mut address_infos = Vec::new();
    // Counter for consecutive unused addresses
    let mut consecutive_unused = 0;
    // Index for address derivation
    let mut index = start;

    // Loop to derive addresses and fetch their information
    while (end.is_none() || index < end.unwrap()) && API_CALL_COUNT.load(Ordering::SeqCst) < MAX_API_CALLS {
        // Create a derivation path for the address
        let derivation_path = DerivationPath::from(vec![
            ChildNumber::Normal { index: 0 },
            ChildNumber::Normal { index },
        ]);

        // Derive the child public key from the extended public key
        let child_pubkey = extended_pubkey.derive_pub(&secp, &derivation_path)?;

        // Determine the address format based on the key type
        let address = match key_type {
            ExtendedKeyType::Xpub => Address::p2pkh(&child_pubkey.public_key, network),
            ExtendedKeyType::Ypub => Address::p2shwpkh(&child_pubkey.public_key, network)?,
            ExtendedKeyType::Zpub | ExtendedKeyType::Vpub => Address::p2wpkh(&child_pubkey.public_key, network)?,
        };

        let address_str = address.to_string();

        // Check if the address has received funds
        match get_address_info(client, &address_str, network_type).await {
            Ok((balance, tx_count)) => {
                // Increment the API call count
                API_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
                println!(
                    "Checking address {}: Balance = {} satoshis, Transactions = {}",
                    address_str, balance, tx_count
                );

                if balance == 0 && tx_count == 0 {
                    consecutive_unused += 1;
                } else {
                    consecutive_unused = 0;
                    address_infos.push(AddressInfo {
                        address: address_str,
                        balance,
                        tx_count,
                    });
                }

                // Break the loop if the unused threshold is reached
                if consecutive_unused >= unused_threshold {
                    break;
                }
            }
            // Handle errors while fetching address information
            Err(e) => {
                eprintln!("Failed to fetch info for address {}: {:?}", address_str, e);
            }
        }

        index += 1;
    }

    Ok(address_infos)
}
