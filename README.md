# Solana Account Historical Balance Fetcher

A Rust debugging tool to fetch the exact SOL balance of a Solana account at a specific historical slot.

## Usage

1. Edit `src/main.rs` and set:

   - `account`: Solana account address
   - `slot`: Slot number to check

2. Run:

```bash
cargo run
```

## Example

```rust
let account: &str = "9msRtBSGQGj4xsbnHpTuqS5Uu99LqrS6ejnMx8ki7Svy";
let slot: u64 = 381150092;
```

Output:

```
✓ Account found in block at slot 381150092
Transaction signature: 3spD3hhXFGzDAf6g9PJrajWcD5SecYtPLb8heULPyTESYhSef2csWyapmRUEWK5nWfrSXWvFxTGTQDP9BoqrUXgq
Lamports: 31667098
SOL Balance: 0.031667098 SOL
```

## Note

Only works for accounts that participated in transactions at the specified slot. For accounts without transactions, an archive node with historical state queries is required.

## License

MIT License - see [LICENSE](LICENSE) file for details
