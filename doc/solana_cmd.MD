## Solana Cmd

### Solana Config

- `solana config get`
  - `solana config set --url ` localhost/devnet/mainnet
    - Anchor.toml Devnet

### Validator

- solana-test-validator
    - `solana-test-validator -r --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metadata.so`
- `rm -rf test-ledger`

```rust
   solana-test-validator -r --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metadata.so --account D4FPEruKEHrG5TenZ2mpDGEfu1iUvTiqBxvpU8HLBvC2 amm_config.json
```

#### Fork Program/Account

- `solana program dump -u m xxxProgramKey metadata.so`
- `solana account -u m xxxAccountKey --output-file amm_config.json --output json-compact`

### Airdrop

- `solana airdrop` 100 {YOUR_WALLET_ADDRESS}

### Solana Sub Cmd

-  `solana balance`
  -  xxxPubKey  `--url mainnet-beta`
-  `solana program extend` PROGRAM_ID   MORE_BYTES
    - `solana program extend xxxPubKey 20000`
-  `solana account`  xxxPubKey

## Spl-Token Cmd

- `spl-token supply` HkLAURj9gSKDYrZiEtcZ7zJY5snH1DyDPKRHxAvpgate
- `spl-token account-info` --address ATAddress
- `spl-token balance` --address ATAddress

## Key Generator

- `Solana-keygen grind --starts-with bnbn:1`
- Create New Pubkey
  - `solana-keygen new -o ./target/deploy/your_program-keypair.json`

## Log

- `export RUST_BACKTRACE=full`
    - `export RUST_BACKTRACE=1`
- `solana logs | grep "address"`
- solana logs | grep "6cZ1ohJeqa9NfG3kBJuScUgAuociq4dxFyztvhoM4A9a"
