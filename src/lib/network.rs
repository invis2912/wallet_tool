// Import necessary Bitcoin utilities and dependencies
use bitcoin::util::bip32::ExtendedPubKey;
use bitcoin::util::base58;
use std::str::FromStr;
use std::error::Error;

// Enum to represent the Bitcoin network type
pub enum NetworkType {
    Mainnet,
    Testnet,
}

// Enum to represent the type of extended public key
#[derive(Debug)]
pub enum ExtendedKeyType {
    Xpub,
    Ypub,
    Zpub,
    Vpub,
}

// Implement Display trait for ExtendedKeyType to convert it to a string
impl std::fmt::Display for ExtendedKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ExtendedKeyType::Xpub => write!(f, "xpub"),
            ExtendedKeyType::Ypub => write!(f, "ypub"),
            ExtendedKeyType::Zpub => write!(f, "zpub"),
            ExtendedKeyType::Vpub => write!(f, "vpub"),
        }
    }
}

// Trait to define the conversion of different extended public keys to xpub
pub trait ExtendedKeyConverter {
    fn convert_to_xpub(&self, pubkey: &str) -> Result<ExtendedPubKey, Box<dyn Error>>;
}

// Implement the ExtendedKeyConverter trait for ExtendedKeyType
impl ExtendedKeyConverter for ExtendedKeyType {
    fn convert_to_xpub(&self, pubkey: &str) -> Result<ExtendedPubKey, Box<dyn Error>> {
        match self {
            ExtendedKeyType::Xpub => ExtendedPubKey::from_str(pubkey).map_err(|e| e.into()),
            ExtendedKeyType::Ypub => {
                let data = base58::from_check(pubkey)?;
                let mut data = data.to_vec();
                data[0..4].copy_from_slice(&[0x04, 0x88, 0xB2, 0x1E]);
                Ok(ExtendedPubKey::decode(&data)?)
            },
            ExtendedKeyType::Zpub => {
                let mut data = base58::from_check(pubkey)?;
                data[0..4].copy_from_slice(&[0x04, 0x88, 0xB2, 0x1E]);
                Ok(ExtendedPubKey::decode(&data)?)
            },
            ExtendedKeyType::Vpub => convert_vpub_to_tpub(pubkey),
        }
    }
}

// Function to parse the network type and key type from the provided extended public key
pub fn parse_network_and_key_type(pubkey: &str) -> Result<(NetworkType, ExtendedKeyType), Box<dyn Error>> {
    let (network_type, key_type) = if pubkey.starts_with("vpub") {
        (NetworkType::Testnet, ExtendedKeyType::Vpub)
    } else if pubkey.starts_with("xpub") {
        (NetworkType::Mainnet, ExtendedKeyType::Xpub)
    } else if pubkey.starts_with("ypub") {
        (NetworkType::Mainnet, ExtendedKeyType::Ypub)
    } else if pubkey.starts_with("zpub") {
        (NetworkType::Mainnet, ExtendedKeyType::Zpub)
    } else {
        return Err("Unsupported pubkey format".into());
    };
    Ok((network_type, key_type))
}

// Function to convert a vpub to tpub (testnet xpub)
fn convert_vpub_to_tpub(vpub: &str) -> Result<ExtendedPubKey, Box<dyn Error>> {
    let mut data = base58::from_check(vpub)?;

    // Change version bytes from vpub (0x045f1cf6) to tpub (0x04358394)
    data[0..4].copy_from_slice(&[0x04, 0x35, 0x83, 0x94]);

    // Parse the key components manually
    let network = bitcoin::network::constants::Network::Testnet;
    let depth = data[4];
    let parent_fingerprint = bitcoin::util::bip32::Fingerprint::from(&data[5..9]);
    let child_number = bitcoin::util::bip32::ChildNumber::from(u32::from_be_bytes(data[9..13].try_into()?));
    let chain_code = bitcoin::util::bip32::ChainCode::from(&data[13..45]);
    let public_key = bitcoin::PublicKey::from_slice(&data[45..78])?;
    
    // Create the ExtendedPubKey
    let extended_pubkey = ExtendedPubKey {
        network,
        depth,
        parent_fingerprint,
        child_number,
        chain_code,
        public_key,
    };

    Ok(extended_pubkey)
}
