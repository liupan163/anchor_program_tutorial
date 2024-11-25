## 版本 锁定

- 第一步，锁定核心工具版本。 避免奇怪case

### rust 版本

- rustup toolchain list
- rustc --version
- rustup install
- rustup default 1.70.0
- rustup toolchain uninstall <version>

#### cargo-build-sbf --version

- check the version of rustc from Solana Tools, run:
    - `https://solana.stackexchange.com/questions/7077/anchor-build-says-cannot-be-built-because-it-requires-rustc-1-68-0-or-newer-bu`

### solana 版本

- solana -V
- sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
- sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"

```
    报错：package solana-program v1.18.17 cannot be built because it requires rustc 1.75.0 or newer, 
    while the currently active rustc version is 1.68.0-dev Either upgrade to rustc
    原因： Anchor detecting wrong Rust version
```

- solana-install init 1.18.25
- solana-install init 1.17.34

### anchor & AVM 版本

- anchor --version
- anchor-cli 0.30.1
    - cargo uninstall anchor-cli

#### 常见错误 CommonIssue

- avm install 0.30.1 is failing
    - *install the tool-chain*
    - rustup install 1.78.0
    - use the toolchain
    - rustup default 1.78.0

## 网络 调整

- Local Env ->  Live Cluster
    - 两个需要调整：
        - 1 solana config set --url localhost
        - 1 solana config set --url devnet
        - 2 Anchor.toml Devnet

## Local 运行 验证节点

- rm -rf test-ledger
- solana-test-validator
    - `solana-test-validator -r --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metadata.so`

```
   solana-test-validator -r --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metadata.so --bpf-program
   CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C cp_swap.so --account D4FPEruKEHrG5TenZ2mpDGEfu1iUvTiqBxvpU8HLBvC2
   amm_config.json --account DNXgeM9EiiaAbaWvwjHj9fQQLAX5ZsfHyvmYUNRAdNC8 fee_receiver.json --account
   GpMZbSM2GgvTKHJirzeGfMFoaZ8UR2X7F4v8vHTvxFbL authority.json
```

### fork 合约Program 到本地

- `solana program dump -u m metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metadata.so`

### fork 账户Account 到本地

- `solana account -u m D4FPEruKEHrG5TenZ2mpDGEfu1iUvTiqBxvpU8HLBvC2 --output-file amm_config.json --output json-compact`

### 空投

- solana airdrop 100 {YOUR_WALLET_ADDRESS}
- solana airdrop 100 GTx4x6mqoNydgvVUvHfhenTCoCyhdKZ9gzYtY4zRMLzf

## Anchor

### 编译build & 部署deploy

- anchor build -p mint_program
- anchor deploy -p mint_program

### 同步 program_id 与 Anchor密钥

- anchor keys list
- anchor keys sync
    - 参考文件位置： `.config/solana/id.json`

### 测试

- anchor clean
- anchor run test
    - anchor run test_create
    - anchor run test_mint_program
- anchor test --skip-local-validator
    - `anchor test --skip-local-validator --skip-deploy`
- solana-keygen pubkey target/deploy/anchor_demo-keypair.json
- solana balance $(solana-keygen pubkey target/deploy/anchor_demo-keypair.json) --url mainnet-beta

### 查看账户大小

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

### CommonIssue 账户太小

- Case: "account data too small for instruction"
    - solana program extend <PROGRAM_ID> <MORE_BYTES>
    - `solana program extend 6cZ1ohJeqa9NfG3kBJuScUgAuociq4dxFyztvhoM4A9a 20000`
    - `RESP: Extended Program Id 6cZ1ohJeqa9NfG3kBJuScUgAuociq4dxFyztvhoM4A9a by 20000 bytes`

## log

- export RUST_BACKTRACE=full
    - export RUST_BACKTRACE=1
- solana logs | grep "address"
- solana logs | grep "6cZ1ohJeqa9NfG3kBJuScUgAuociq4dxFyztvhoM4A9a"
- solana logs | grep "24PNhTaNtomHhoy3fTRaMhAFCRj4uHqhZEEoWrKDbR5p"

### Anchor error Code

- `https://docs.rs/anchor-lang/latest/anchor_lang/error/enum.ErrorCode.html`

## Other Issue 其他遇到过的问题

### 引入 其他crate

- `cargo add mpl-token-metadata@1.13.1 --package mint_program`

## Net & Account & NFT ---> Fork

- https://www.quicknode.com/guides/solana-development/accounts-and-data/fork-programs-to-localnet#overview

### MacOS compatible bug

- Error checking to unpack genesis archive: Archive error: extra entry found: "._genesis.bin" Regular
    - https://github.com/solana-labs/solana/issues/35648
