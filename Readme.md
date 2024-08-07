# Wallet Balance Discovery Tool

This CLI tool and code wrtitten in Rust derives Bitcoin addresses from an extended public key (xpub/ypub/zpub/vpub) and retrieves the wallet address, balances and transaction counts associated with that key.

xpub - BTC Mainnet using P2PKH

ypub - BTC Mainnet using P2SHWPKH

zpub - BTC Mainnet using P2WPKH

vpub - BTC Testnet using P2WPKH

It can be used to analyze Bitcoin wallets and determine the total balance held across derived addresses.

## Features

- Derive Bitcoin addresses from different extended public keys for mainnet and testnet.
- Auto-selection of Mainnet or Testnet based on Derived keys.
- Fetch balance and transaction count for each derived address.
- Total BTC held and transaction counts across Derived Wallets.
- Option to choose more depth can bring more derived wallets in visibility.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (ensure `cargo` is available in your PATH)

### Build and Install

1. Clone the repository:

    ```sh
    git clone <repository-url>
    cd wallet_tool
    ```

2. Build and install the CLI tool:

    ```sh
    cargo install --path .
    ```

This command will build the project and install the binary in your Cargo bin directory (usually `~/.cargo/bin`).

## Usage

Once installed, you can run the CLI tool using the following command:

```sh
wallet_tool --pubkey <extended-public-key> --start <start-index> --end <end-index> --unused_threshold <threshold>

Arguments
--pubkey: The extended public key (xpub/ypub/zpub/vpub).
[optional] --start: The starting index for deriving addresses (default: 0).
[optional] --end: The ending index for deriving addresses (default: 0, which means it will run until the unused threshold is reached).
[optional] --unused_threshold: The number of unused addresses in a row to stop derivation (default: 80).


or if you have all the source file, you can build the project using: cargo build and then cargo run <extended-public-key>

```

Example:

```sh
wallet_tool --pubkey xpub6CUGRUonZSQ4TWtTMmzXdrXDtypWKiKrhko4egpiMZbpiaQL2jkwSB1icqYh2cfDfVxdx4df189oLKnC5fSwqPfgyP3hooxujYzAu3fDVmz --start 0 --end 0 --unused_threshold 100

or

wallet_tool --pubkey xpub6CUGRUonZSQ4TWtTMmzXdrXDtypWKiKrhko4egpiMZbpiaQL2jkwSB1icqYh2cfDfVxdx4df189oLKnC5fSwqPfgyP3hooxujYzAu3fDVmz
```

Output

```sh
The tool will output the following information for each used address:

Address
Balance in satoshis and BTC
Number of transactions

At the end, it will also print the total number of used addresses, the total balance in satoshis and BTC, and the total number of transactions.

```

Project Structure

```sh

wallet_tool
├── Cargo.toml
└── src
    ├── main.rs
    ├── lib
    │   ├── mod.rs
    │   ├── constants.rs
    │   ├── error.rs
    │   ├── network.rs
    │   ├── address.rs
    │   ├── client.rs
    │   └── cli.rs

main.rs: Entry point of the application.
lib/mod.rs: Library module definition.
lib/constants.rs: Constants used in the project.
lib/error.rs: Custom error definitions.
lib/network.rs: Network and key type parsing.
lib/address.rs: Address derivation and information retrieval.
lib/client.rs: HTTP client setup.
lib/cli.rs: CLI argument parsing.
```



## Demo

```sh
ajeet@Ajeets-MBP wallet_tool % cargo run xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8

Running `target/debug/wallet_tool xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8`
  ____    _   _                    _            __        __          _   _          _       _____                   _ 
 | __ )  (_) | |_    ___    ___   (_)  _ __     \ \      / /   __ _  | | | |   ___  | |_    |_   _|   ___     ___   | |
 |  _ \  | | | __|  / __|  / _ \  | | | '_ \     \ \ /\ / /   / _` | | | | |  / _ \ | __|     | |    / _ \   / _ \  | |
 | |_) | | | | |_  | (__  | (_) | | | | | | |     \ V  V /   | (_| | | | | | |  __/ | |_      | |   | (_) | | (_) | | |
 |____/  |_|  \__|  \___|  \___/  |_| |_| |_|      \_/\_/     \__,_| |_| |_|  \___|  \__|     |_|    \___/   \___/  |_|

Checking address 12CL4K2eVqj7hQTix7dM7CVHCkpP17Pry3: Balance = 0 satoshis, Transactions = 8
Checking address 13Q3u97PKtyERBpXg31MLoJbQsECgJiMMw: Balance = 0 satoshis, Transactions = 0
Checking address 1J4LVanjHMu3JkXbVrahNuQCTGCRRgfWWx: Balance = 0 satoshis, Transactions = 0
Checking address 1EBPs7ApVkRNy9Y8Z8xLAueeH4wuD1Aixb: Balance = 0 satoshis, Transactions = 0
Checking address 1H2RCEj5KFAxY4TvibjKivf8sPipZA62CF: Balance = 0 satoshis, Transactions = 0
Checking address 1K6rDJZ54hn4XouChMSp1zcZN5vniP2fzw: Balance = 0 satoshis, Transactions = 0
Checking address 1MGxajmnvNKW84o72fRynwzrDXj7htJYBo: Balance = 0 satoshis, Transactions = 0
Checking address 1H4STVrrTCPR6qiL7qFUXq97CBr3DHPwxD: Balance = 0 satoshis, Transactions = 0
Checking address 1JLhjtQ9E5GyT9HARXc3xFxD9pYvgLzVU2: Balance = 0 satoshis, Transactions = 0
Checking address 15MbJzwHGPq5ETKLBp3yPHoxQ5GUB9avyS: Balance = 0 satoshis, Transactions = 2
Checking address 17fz4VHcFtJxS4GNoM2AkY76ZXkPUXUKXy: Balance = 0 satoshis, Transactions = 0
Checking address 1KgnZTQTagyR7j6quqFnQAecUooPoTZx9K: Balance = 0 satoshis, Transactions = 0
Checking address 17PmXRcXGxBXWWhqJeB94cfwbPmy1x9fN: Balance = 0 satoshis, Transactions = 0
Checking address 1JgcEGhuFg4Aofmd6HbhCvxBXTPnSY6tXw: Balance = 0 satoshis, Transactions = 0
Checking address 1G93sV7huAgSBfzzxi5eTeyq2VXL2MogBS: Balance = 0 satoshis, Transactions = 0

Address: 12CL4K2eVqj7hQTix7dM7CVHCkpP17Pry3 - Balance: 0 satoshis (0.00000000 BTC) - Transactions: 8
Address: 15MbJzwHGPq5ETKLBp3yPHoxQ5GUB9avyS - Balance: 0 satoshis (0.00000000 BTC) - Transactions: 2
Address: 16nWB7Si2hTUtqi71dXtFBreScGWfhyNnm - Balance: 0 satoshis (0.00000000 BTC) - Transactions: 2
Address: 1JEYhhAGC2JkLJhdnC1tWk2CtH64sX2Ur8 - Balance: 0 satoshis (0.00000000 BTC) - Transactions: 4
Total Used Addresses: 4
Total Balance: 0 satoshis (0.00000000 BTC), Total Transactions: 16
Thank you for using the Tool

```
## License

[MIT](https://choosealicense.com/licenses/mit/)


## Appendix

API calls to Blockstream.info : https://blockstream.info

https://www.blockchain.com/explorer

https://bitcoinlib.readthedocs.io

https://blockpath.com

https://asecuritysite.com

https://electrum.readthedocs.io/en/latest/xpub_version_bytes.html


