## Lock Dependency Version

- rustc -V
- solana -V
- anchor -V
- Anchor.toml, 锁定anchor、solana版本

### rust Version

- rustup toolchain list
  - rustc --version
  - rustup toolchain uninstall <version>
- rustup install
- rustup override set 1.79
  - special directory level.
- rustup default 1.70.0

### solana Version

- solana -V
  - list  `ls ~/.local/share/solana/install/releases/`
- sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
- sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
- solana-install init 1.18.25
- solana-install init 1.17.34

### anchor & AVM Version

- anchor --version
- avm list
- avm install 0.28.0
- avm use 0.28.0 --local
  - avm use 0.28.0
- anchor-cli 0.30.1
    - cargo uninstall anchor-cli

#### 常见错误 CommonIssue

- avm install 0.30.1 is failing
    - *install the tool-chain*
    - rustup install 1.78.0
    - use the toolchain
    - rustup default 1.78.0

## Env NetType

- Local Env ->  Live Cluster
    - Two changes：
        - 1 solana config set --url localhost
        - 1 solana config set --url devnet
        - 2 Anchor.toml Devnet

## Local run Validator

- rm -rf test-ledger
- solana-test-validator
    - `solana-test-validator -r --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metadata.so`

```rust
   solana-test-validator -r --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metadata.so --bpf-program
   CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C cp_swap.so --account D4FPEruKEHrG5TenZ2mpDGEfu1iUvTiqBxvpU8HLBvC2
   amm_config.json --account DNXgeM9EiiaAbaWvwjHj9fQQLAX5ZsfHyvmYUNRAdNC8 fee_receiver.json --account
   GpMZbSM2GgvTKHJirzeGfMFoaZ8UR2X7F4v8vHTvxFbL authority.json
```

### fork Program to LocalEnv

- `solana program dump -u m metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metadata.so`

### fork AccountInfo to LocalEnv

- `solana account -u m D4FPEruKEHrG5TenZ2mpDGEfu1iUvTiqBxvpU8HLBvC2 --output-file amm_config.json --output json-compact`

### Airdrop to AccountBalance

- solana airdrop 100 {YOUR_WALLET_ADDRESS}
- solana airdrop 100 GTx4x6mqoNydgvVUvHfhenTCoCyhdKZ9gzYtY4zRMLzf

## Anchor

### build & deploy

- anchor build -p mint_program
- anchor deploy -p mint_program
- `anchor deploy --provider.cluster mainnet -- --with-compute-unit-price 40`

### SYNC: program_id * AnchorPrivateKey

- anchor keys list
- anchor keys sync
    - 参考文件位置： `.config/solana/id.json`

### Test

- anchor clean
- anchor run test
    - anchor run test_create
    - anchor run test_mint_program
- anchor test --skip-local-validator
    - `anchor test --skip-local-validator --skip-deploy`
- solana-keygen pubkey target/deploy/anchor_demo-keypair.json
- solana balance $(solana-keygen pubkey target/deploy/anchor_demo-keypair.json) --url mainnet-beta

### Check Account Size

- `realloc::zero = false`
- solana address
- solana account 地址
    - node -e 'console.log(require("bs58").decode("   ATAddress   "))'
        - 前两行 mintPubKey
        - 接下来两行 tokenOwner
        - 接下来 tokenBalance
    - node -e 'console.log(require("bs58").decode("HkLAURj9gSKDYrZiEtcZ7zJY5snH1DyDPKRHxAvpgate"))'
- spl-token supply HkLAURj9gSKDYrZiEtcZ7zJY5snH1DyDPKRHxAvpgate
- spl-token account-info --address ATAddress
- spl-token balance --address ATAddress

### Common Issue

- Case: "account data too small for instruction" 账户太小
    - solana program extend <PROGRAM_ID> <MORE_BYTES>
    - `solana program extend 6cZ1ohJeqa9NfG3kBJuScUgAuociq4dxFyztvhoM4A9a 20000`
    - `RESP: Extended Program Id 6cZ1ohJeqa9NfG3kBJuScUgAuociq4dxFyztvhoM4A9a by 20000 bytes`
- Safety checks failed: Failed to get program path
  - https://solana.stackexchange.com/questions/14900/suddenly-getting-error-message-safety-checks-failed-failed-to-get-program-path

## log

- export RUST_BACKTRACE=full
    - export RUST_BACKTRACE=1
- solana logs | grep "address"
- solana logs | grep "6cZ1ohJeqa9NfG3kBJuScUgAuociq4dxFyztvhoM4A9a"
- solana logs | grep "24PNhTaNtomHhoy3fTRaMhAFCRj4uHqhZEEoWrKDbR5p"

### Anchor error Code

- `https://docs.rs/anchor-lang/latest/anchor_lang/error/enum.ErrorCode.html`

## Program CMD

- Upgrade related Accounts
  - 保存 好两个keypair.json
    - `target/deploy/**.json` 通常对应 合约地址
    - 默认升级权限 地址 `~/.config/solana/id.json`
  - `anchor deploy `
- Close intermediate accounts 关闭中间账户
  - `solana-keygen recover -o buffer-keypair.json ` and then input mnemonic
  - `solana program close --url http://127.0.0.1:8899   --keypair ~/.config/solana/id.json --buffers  --config ./buffer-keypair.json`
  - `solana program close AkmUVkvRE4ZkRoo9NonV8zA99GRjoYXUDp2s5K9eDQHm `
    - `--recipient E8aQ18Xbr8FgKq5M7cQnuvJUwU8ZsningNLhcEDAPD8Y`
    - `--url https://api.mainnet-beta.solana.com`
- Transfer Authority
  - solana program set-upgrade-authority ProgramId --keypair ~/.config/solana/id.json   --new-upgrade-authority  NewAuthority --url http://127.0.0.1:8899
    - NOTE careful: `--skip-new-upgrade-authority-signer-check`
- Not upgradable Program 不可升级合约
  - `solana program deploy --final xxx.so`
- Close Program
  - `solana program close ProgramId --keypair ~/.config/solana/id.json --url http://127.0.0.1:8899`
- Check Program Pubkey
  - `solana address -k target/deploy/xxx_program-keypair.json`
- Create New Pubkey
  - `solana-keygen new -o ./target/deploy/your_program-keypair.json`

## Other Issue 其他遇到过的问题

### Import other crate

- `cargo add mpl-token-metadata@1.13.1 --package mint_program`

## Net & Account & NFT ---> Fork

- https://www.quicknode.com/guides/solana-development/accounts-and-data/fork-programs-to-localnet#overview

### MacOS compatible bug

- Error checking to unpack genesis archive: Archive error: extra entry found: "._genesis.bin" Regular
    - https://github.com/solana-labs/solana/issues/35648
