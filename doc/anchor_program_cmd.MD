## Anchor Cmd

### build & deploy

- `anchor build`
  - anchor build -p mint_program
- `anchor deploy`
  - anchor deploy -p mint_program

### program_id

- Sync ProgramId: 
  - `anchor keys sync`
    -  .config/solana/id.json
    -  anchor keys list
- Output ProgramId: 
  - `solana address -k target/deploy/id.keypair`
  - `solana-keygen pubkey target/deploy/anchor_demo-keypair.json`

### Test

- anchor clean
- `anchor run test`
    - anchor run test_create
    - anchor run test_mint_program
- `anchor test --skip-local-validator --skip-deploy`

### Check Account Size

- `realloc::zero = false`
- solana address
- solana account 地址

## Program Manage Cmd

- deploy
  - `anchor deploy --provider.cluster mainnet -- --with-compute-unit-price 40`
- Upgrade
  - `anchor deploy`
  - Program Address and Private Key: `target/deploy/**.json`
  - Program Authority PrivateKey(default): `~/.config/solana/id.json`
- Close intermediate accounts 
  - step1: `solana-keygen recover -o buffer-keypair.json `
  - step2: `solana address -k buffer-keypair.json`
  - step3: `solana program close xxxProgramKey --recipient yyyRecipientKey`
    - Or `solana program close --keypair /Users/liupan/.config/solana/id.json  --buffers  --config ./buffer-keypair.json`
      -  `--url https://api.mainnet-beta.solana.com`
      -  `--url http://127.0.0.1:8899`
-  Transfer Authority
   -  `solana program set-upgrade-authority ProgramKey --keypair ~/.config/solana/id.json --new-upgrade-authority NewAuthority`
      - NOTE careful: `--skip-new-upgrade-authority-signer-check`
      -  `--url https://api.mainnet-beta.solana.com`
      -  `--url http://127.0.0.1:8899`
- Not upgradable Program
  - `solana program deploy --final xxx.so`
- Close Program
  - `solana program close ProgramId --keypair ~/.config/solana/id.json`
      -  `--url https://api.mainnet-beta.solana.com`
      -  `--url http://127.0.0.1:8899`
