use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcBlockConfig, TransactionDetails};
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::UiTransactionEncoding;
use std::str::FromStr;

fn main() {
    // === INPUTS ===
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let account: &str = "9msRtBSGQGj4xsbnHpTuqS5Uu99LqrS6ejnMx8ki7Svy";
    let slot: u64 = 381150092;

    let pubkey = Pubkey::from_str(account).expect("Invalid pubkey");

    // === RPC Client ===
    let client = RpcClient::new(rpc_url.to_string());

    println!(
        "Fetching account balance for {} at exact slot {}...\n",
        account, slot
    );

    // === Fetch block at specific slot ===
    // Use Full to get complete transaction data with account balances
    let block_config = RpcBlockConfig {
        encoding: Some(UiTransactionEncoding::Json),
        transaction_details: Some(TransactionDetails::Full),
        rewards: Some(false),
        max_supported_transaction_version: Some(0),
        ..RpcBlockConfig::default()
    };

    let block = client
        .get_block_with_config(slot, block_config)
        .expect("Failed to fetch block at specified slot");

    // === Search for account in block's transaction account data ===
    let mut account_balance: Option<u64> = None;
    let mut found_tx_signature: Option<String> = None;

    if let Some(transactions) = &block.transactions {
        println!("Checking {} transactions in block...", transactions.len());

        // Search all transactions for our account
        for tx in transactions.iter() {
            if let Some(meta) = &tx.meta {
                // Handle different transaction encodings
                let account_keys: Vec<String> = match &tx.transaction {
                    solana_transaction_status::EncodedTransaction::Json(json_tx) => {
                        match &json_tx.message {
                            solana_transaction_status::UiMessage::Parsed(parsed) => parsed
                                .account_keys
                                .iter()
                                .map(|acc| acc.pubkey.clone())
                                .collect(),
                            solana_transaction_status::UiMessage::Raw(raw) => {
                                raw.account_keys.clone()
                            }
                        }
                    }
                    _ => vec![],
                };

                // Find our account in this transaction
                for (idx, key_str) in account_keys.iter().enumerate() {
                    if let Ok(key_pubkey) = Pubkey::from_str(key_str) {
                        if key_pubkey == pubkey {
                            if idx < meta.pre_balances.len() {
                                account_balance = Some(meta.pre_balances[idx]);
                                // Extract transaction signature
                                if let solana_transaction_status::EncodedTransaction::Json(
                                    json_tx,
                                ) = &tx.transaction
                                {
                                    if let Some(sig) = json_tx.signatures.first() {
                                        found_tx_signature = Some(sig.clone());
                                    }
                                }
                                break;
                            }
                        }
                    } else if key_str == account {
                        // String match fallback
                        if idx < meta.pre_balances.len() {
                            account_balance = Some(meta.pre_balances[idx]);
                            // Extract transaction signature
                            if let solana_transaction_status::EncodedTransaction::Json(json_tx) =
                                &tx.transaction
                            {
                                if let Some(sig) = json_tx.signatures.first() {
                                    found_tx_signature = Some(sig.clone());
                                }
                            }
                            break;
                        }
                    }
                }
            }

            if account_balance.is_some() {
                break;
            }
        }
    }

    match account_balance {
        Some(lamports) => {
            let sol = lamports as f64 / 1_000_000_000f64;
            println!("\n✓ Account found in block at slot {}", slot);
            if let Some(tx_sig) = found_tx_signature {
                println!("Transaction signature: {}", tx_sig);
            }
            println!("Lamports: {}", lamports);
            println!("SOL Balance: {:.9} SOL", sol);
        }
        None => {
            println!(
                "\n✗ Account {} was not found in any transactions at slot {}.",
                account, slot
            );
            println!(
                "\nNote: This method only works for accounts that participated in transactions at the specified slot."
            );
            println!("For accounts that didn't have transactions in this block, you need:");
            println!("  1. An archive node RPC endpoint (e.g., Helius, QuickNode, or self-hosted)");
            println!("  2. Or query the account state from a block where it had a transaction");
        }
    }
}
