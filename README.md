## 版本检查

solana --version
anchor --version

## 配置 Solana 在本地主机上运行

solana config set --url localhost

## 运行测试验证节点

- rm -rf test-ledger
- solana-test-validator
    - solana-test-validator -r --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metadata.so

## 空投

solana airdrop 100 {YOUR_WALLET_ADDRESS}
solana airdrop 100 GTx4x6mqoNydgvVUvHfhenTCoCyhdKZ9gzYtY4zRMLzf

# Anchor

anchor build

- anchor deploy
    - Case: "account data too small for instruction"
        - solana program extend <PROGRAM_ID> <MORE_BYTES>
        - solana program extend 6cZ1ohJeqa9NfG3kBJuScUgAuociq4dxFyztvhoM4A9a 20000
            - RESP: Extended Program Id 6cZ1ohJeqa9NfG3kBJuScUgAuociq4dxFyztvhoM4A9a by 20000 bytes

## 确保 program_id 与 Anchor 密钥同步

- anchor keys list
- anchor keys sync

.config/solana/id.json

## 测试

- anchor clean
- anchor run test
    - anchor run test_create
    - anchor run test_mint_program
- anchor test --skip-local-validator
    - anchor test --skip-local-validator --skip-deploy

### ProgramID not match

- solana-keygen pubkey target/deploy/anchor_demo-keypair.json
- solana balance $(solana-keygen pubkey target/deploy/anchor_demo-keypair.json) --url mainnet-beta

### Rent

- solana rent <字节数>
- [anchor Rent](https://docs.rs/solana-program/latest/solana_program/rent/index.html)
    - `use anchor_lang::solana_program::rent as rent_module;`
        - `rent_module::ACCOUNT_STORAGE_OVERHEAD`
        - `rent_module::DEFAULT_LAMPORTS_PER_BYTE_YEAR`
        - `rent_module::DEFAULT_EXEMPTION_THRESHOLD`

### 查看账户大小

- `realloc::zero = false`
- solana address
- solana account 地址

## log

- export RUST_BACKTRACE=full
    - export RUST_BACKTRACE=1
- solana logs | grep "address"
- solana logs | grep "6cZ1ohJeqa9NfG3kBJuScUgAuociq4dxFyztvhoM4A9a"
- solana logs | grep "24PNhTaNtomHhoy3fTRaMhAFCRj4uHqhZEEoWrKDbR5p"

## Anchor error Code

https://docs.rs/anchor-lang/latest/anchor_lang/error/enum.ErrorCode.html

## 其他crate

cargo add mpl-token-metadata@1.13.1 --package mint_program

## Net & Account & NFT ---> Fork

https://www.quicknode.com/guides/solana-development/accounts-and-data/fork-programs-to-localnet#overview

### MacOS compatible bug

- Error checking to unpack genesis archive: Archive error: extra entry found: "._genesis.bin" Regular
    - https://github.com/solana-labs/solana/issues/35648