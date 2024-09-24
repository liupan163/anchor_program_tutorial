## 配置 Solana 在本地主机上运行

solana config set --url localhost

## 运行测试验证节点

solana-test-validator

solana-test-validator --reset

## 空投

solana airdrop 100 {YOUR_WALLET_ADDRESS}
solana airdrop 100 GTx4x6mqoNydgvVUvHfhenTCoCyhdKZ9gzYtY4zRMLzf

# Anchor

anchor build

anchor deploy

## 确保 program_id 与 Anchor 密钥同步

anchor keys sync

## 测试

- anchor clean
- anchor run test
    - anchor run test_create
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

- solana logs | grep "address"