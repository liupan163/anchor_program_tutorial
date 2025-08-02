# Anchor Development Notes

- This repository serves as a reference for:
  - Common issues and solutions encountered when working with the Anchor framework, development tools, and environment setup.
  - Key features and typical use cases of SPL Tokens.
  - Common business logic patterns and development tips when using Anchor.
  - Pitfalls frequently encountered during development and corresponding optimization strategies.
  - Usage scenarios for helpful utility crates, macros, and third-party libraries.
  - Collect and organize utility files for creating instructions/transactions, handling keypairs and public keys, and other common development tasks.

## Development Tool / Environment Setup

> Note: Lock the versions of related tools in advance to avoid unexpected environment errors.

- Anchor.toml
  - Lock Tools Version in `[toolchain]`
- `rustc --version`
- `solana --version`
- `anchor --version`

### Install and Management Tools Doc

-  [Install and Management Tools Doc](./doc/install_tools.MD)

### Common Cmd Referce Doc

- [Solana/SplToken/KeyGen related Commands Doc](./doc/solana_cmd.MD)
- [Program lifetime related Commands Doc](./doc/anchor_program_cmd.MD)
- [Anchor related Commands Doc](./doc/anchor_program_cmd.MD)

## Core Features and Business Scenarios

- Define and use common `Account Types`, along with related Anchor macro constraints.
- Handle `remaining_accounts` and account lifetimes:
  - Convert raw AccountInfo into specific Structured Type as needed.
  - Group and manage accounts based on usage to avoid the 64-account limit during instruction execution.
  - Apply similar strategies when performing CPI (Cross-Program Invocation) to bypass the same limitation.
- `Managing On-Chain Data Size` and Converting Between Raw Data and Structured Types
  - usage of `bytemuck` and `ZeroCopy`

## Verifiable Program

- [Anchor official Doc](https://www.anchor-lang.com/docs/references/verifiable-builds)
  - PS: one pitfall, upload IDL at same time!!! 
    - or verify will fail by official verify method.